use anyhow::{bail, Result};
use clap::{command, Parser, Subcommand};
use crossterm::{
    cursor::{Hide, MoveDown, MoveLeft, MoveRight, MoveUp, RestorePosition, SavePosition, Show},
    event::{Event, KeyCode, KeyEvent},
    execute,
    style::{Color, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::{distributions::{Alphanumeric, Uniform}, thread_rng, Rng};
use scorched_earth_network::{Connection, MoveMessage};

use std::{
    ffi::OsString,
    io::{stdout, Write},
    sync::mpsc::channel, time::Duration,
};

use scorched_earth_core::{Board, Direction, Move, PlayerColor, TileContents, Vector, BOARD_SIZE};

#[derive(Debug, Parser)]
#[command(name = "scorched_earth_tui")]
#[command(about = "TUI for the game Scorched Earth", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long)]
    relay: Option<OsString>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Host,
    #[command(arg_required_else_help = true)]
    Join {
        id: OsString,
    },
}

fn player_term_color(color: PlayerColor) -> Color {
    match color {
        PlayerColor::Blue => Color::Blue,
        PlayerColor::Cyan => Color::Cyan,
        PlayerColor::Yellow => Color::Yellow,
        PlayerColor::Green => Color::Green,
        PlayerColor::Magenta => Color::Magenta,
    }
}

// Initial setup for drawing moves
fn setup_drawing(board: &Board) -> crossterm::Result<()> {
    // Disables typing to the terminal so keyboard input isn't visible
    enable_raw_mode()?;
    print!("\n");

    // Hide the cursor
    execute!(stdout(), Hide,)?;

    // Draw the board with a grey border and blank inside
    for i in 0..BOARD_SIZE + 2 {
        for j in 0..BOARD_SIZE + 2 {
            let color = if i == 0 || j == 0 || i == BOARD_SIZE + 1 || j == BOARD_SIZE + 1 {
                Color::DarkMagenta
            } else {
                Color::Reset
            };
            execute!(stdout(), SetBackgroundColor(color))?;
            print!("  ");
        }
        execute!(stdout(), SetBackgroundColor(Color::Reset))?;
        print!("\n\r");
    }

    print!("\n\n");

    // Go back to the top and SAVE THE POSITION OF THE TOP LEFT CORNER (important; this is used
    // for drawing later)
    execute!(stdout(), MoveUp(BOARD_SIZE as u16 + 4), SavePosition)?;

    // Draw the players
    for player in &board.players {
        draw_tile(player.pos, player_term_color(player.color))?;
    }

    Ok(())
}

// Restore the terminal to normal state after the program finishes
fn finish_drawing() -> crossterm::Result<()> {
    execute!(
        stdout(),
        Show,
        RestorePosition,
        MoveDown(BOARD_SIZE as u16 + 10),
    )?;

    disable_raw_mode()
}

// Draw a single cell on the board. 2 characters wide to be more square-shaped
fn draw_tile(pos: Vector, color: Color) -> crossterm::Result<()> {
    execute!(
        stdout(),
        RestorePosition,
        MoveDown((pos.y + 1) as u16),
        MoveRight(((pos.x + 1) * 2) as u16),
        SetBackgroundColor(color)
    )?;

    print!("  ");

    stdout().flush()?;

    Ok(())
}

fn draw_tile_contents(pos: Vector, contents: TileContents) -> crossterm::Result<()> {
    let color = match contents {
        TileContents::Empty => Color::Reset,
        TileContents::Scorched => Color::Red,
        TileContents::Player(c) => player_term_color(c),
    };
    draw_tile(pos, color)
}

// Draw the border around the board a certain color
fn draw_border(color: Color) -> crossterm::Result<()> {
    execute!(stdout(), RestorePosition, SetBackgroundColor(color))?;

    print!("{}", "  ".repeat(BOARD_SIZE + 2));

    for _ in 0..BOARD_SIZE + 1 {
        execute!(stdout(), MoveDown(1), MoveLeft(2),)?;

        print!("  ");
    }

    execute!(
        stdout(),
        RestorePosition,
        MoveRight(2),
        SetBackgroundColor(color),
    )?;

    for _ in 0..BOARD_SIZE + 1 {
        execute!(stdout(), MoveDown(1), MoveLeft(2),)?;

        print!("  ");
    }

    print!("{}", "  ".repeat(BOARD_SIZE + 1));

    stdout().flush()?;

    Ok(())
}

// Fill in the entire board a certain color to show who wins
fn fill_box(color: Color) -> crossterm::Result<()> {
    execute!(stdout(), RestorePosition, SetBackgroundColor(color))?;

    for _ in 0..BOARD_SIZE + 2 {
        print!("{}", "  ".repeat(BOARD_SIZE + 2));
        execute!(stdout(), MoveDown(1), MoveLeft((BOARD_SIZE as u16 + 2) * 2))?;
    }

    Ok(())
}

enum Keypress {
    Dir(Direction),
    Confirm,
    Quit,
}

fn read_key() -> crossterm::Result<Keypress> {
    loop {
        match crossterm::event::read()? {
            // Wait for a keypress and only accept it if it's wasd or q
            Event::Key(KeyEvent {
                code: KeyCode::Char(c @ ('w' | 'a' | 's' | 'd' | 'q' | ' ')),
                ..
            }) => {
                return Ok(match c {
                    'w' => Keypress::Dir(Direction::Up),
                    'a' => Keypress::Dir(Direction::Left),
                    's' => Keypress::Dir(Direction::Down),
                    'd' => Keypress::Dir(Direction::Right),
                    'q' => Keypress::Quit,
                    ' ' => Keypress::Confirm,
                    _ => unreachable!(),
                })
            }
            _ => {}
        }
    }
}

fn run(mut b: Board, mut conn: Option<Connection>) -> Result<()> {
    setup_drawing(&b)?;
    'main: loop {
        let i = b.turn;
        let other_player = (i + 1) % 2;

        // Set the border to show the current player's color
        draw_border(player_term_color(b.players[i].color))?;

        // loop until a valid move is made
        let mut m: Option<Move> = None;
        let (next_move, other_player_board): (Move, Option<Board>) = if let Some(c) =
            conn.as_mut().filter(|c| c.player_num == i)
        {
            // If connected to another player and it's their turn, receive their move over the
            // network instead of making the move locally

            enum WaitResult {
                Move(Result<MoveMessage, scorched_earth_network::Error>),
                Cancelled,
            }

            // Bit of a hack with two channels and threads to let players quit when it's not their
            // turn. The first thread waits for a move and sends WaitResult::Move over tx, while
            // the other thread sends Cancelled if q is pressed. Whichever one happens first is
            // used.
            let other_move = crossbeam::scope(|s| {
                let (tx, rx) = channel();
                let tx2 = tx.clone();
                // Need another channel to cancel the canceller if the receive which would have
                // been cancelled completes
                let (cancel_tx, cancel_rx) = channel();

                s.spawn(move |_| {
                    tx.send(WaitResult::Move(c.recv_move())).expect("failed to get received move");
                    // Tell the other thread to stop now
                    cancel_tx.send(())
                });

                s.spawn(move |_| {
                    while let Err(_) = cancel_rx.try_recv() {
                        if let Ok(true) = crossterm::event::poll(Duration::from_millis(100)) {
                            match crossterm::event::read() {
                                // Wait for a keypress and only accept it if it's q
                                Ok(Event::Key(KeyEvent {
                                    code: KeyCode::Char('q'),
                                    ..
                                })) => {
                                        tx2.send(WaitResult::Cancelled).expect("Failed to send cancel message from pressing q");
                                    }
                                _ => {}
                            }
                        }
                    }
                });

                if let Ok(WaitResult::Move(m)) = rx.recv() {
                    m
                } else {
                    finish_drawing().expect("Failed to reset terminal");
                    std::process::exit(0);
                }
            })
            .expect("Failed to join threads")?;

            if other_move.player != i {
                bail!("Player moved and it isn't their turn!");
            }

            (other_move.new_move, Some(other_move.new_board.clone()))
        } else {
            // Otherwise preview moves in a loop until one is selected locally
            loop {
                let key = read_key()?;

                // Redraw the tile from the last move preview
                if let Some(potential_move) = m {
                    for tile in potential_move.tiles_along_path() {
                        let target_position = b.players[i].pos + tile;
                        if let Some(contents) = b.tile_contents_at(target_position) {
                            draw_tile_contents(target_position, contents)?;
                        }
                    }
                }

                match key {
                    Keypress::Quit => {
                        break 'main;
                    }

                    Keypress::Confirm => {
                        if let Some(valid_move) =
                            m.filter(|potential_move| b.is_move_valid(i, *potential_move))
                        {
                            break (valid_move, None);
                        } else {
                            continue;
                        }
                    }

                    Keypress::Dir(input_dir) => {
                        match m.as_mut() {
                            None => {
                                m = Some(Move {
                                    dir: input_dir,
                                    len: 1,
                                })
                            }
                            Some(old_move) => {
                                if input_dir == old_move.dir && old_move.len == 1 {
                                    old_move.len = 2;
                                } else if input_dir == old_move.dir.opposite() && old_move.len == 2
                                {
                                    old_move.len = 1;
                                } else {
                                    old_move.len = 1;
                                    old_move.dir = input_dir;
                                }
                            }
                        }

                        if let Some(potential_move) = m {
                            let target_position = b.players[i].pos + potential_move.to_vector();
                            if (0..BOARD_SIZE as isize).contains(&target_position.x)
                                && (0..BOARD_SIZE as isize).contains(&target_position.y)
                            {
                                let color = if b.is_move_valid(i, potential_move) {
                                    Color::White
                                } else {
                                    Color::Grey
                                };
                                for tile in potential_move.tiles_along_path() {
                                    draw_tile(b.players[i].pos + tile, color)?;
                                }
                            }
                        }
                    }
                }
            }
        };

        let res = b.make_move(i, next_move);
        for (pos, contents) in res.changes {
            draw_tile_contents(pos, contents)?;
        }

        if let Some(b2) = other_player_board {
            // If the opponent has a board that doesn't match, they're probably cheating or
            // something
            if b != b2 {
                bail!("Other player's board doesn't match!");
            }
        }

        if let Some(c) = conn.as_mut() {
            // If there's another player connected but it's not their turn, send them our move
            if c.player_num == other_player {
                c.send_move(MoveMessage {
                    new_board: b.clone(),
                    new_move: next_move,
                    player: i,
                })?;
            }
        }

        if let Some(color) = res.winner {
            fill_box(player_term_color(color))?;
            break 'main;
        }
    }
    Ok(())
}

fn run_host(addr: &str) -> Result<()> {
    let mut board = Board::default();
    let mut rng = thread_rng();
    board.turn = if rng.gen_bool(0.5) { 1 } else { 0 };
    let mut secret = [0u8; 32];
    for b in &mut secret {
        *b = rng.sample(Uniform::new_inclusive(b'0', b'9'));
    }
    let secret_string = String::from_utf8_lossy(&secret);
    println!("Hosting game with id: {}", secret_string);
    let conn = Connection::host(addr, &secret, &board)?;
    run(board, Some(conn))
}

fn run_join(addr: &str, id: &str) -> Result<()> {
    let (conn, board) = Connection::conn(addr, id.as_bytes())?;
    run(board, Some(conn))
}

fn run_offline() -> Result<()> {
    run(Board::default(), None)
}

fn try_main() -> Result<()> {
    let args = Cli::parse();
    let addr = args.relay.map_or("zorbulator.com:8080".to_string(), |s| {
        s.to_str().expect("invalid relay address").to_string()
    });

    match args.command {
        None => run_offline()?,
        Some(Commands::Host) => run_host(&addr)?,
        Some(Commands::Join { id }) => run_join(&addr, id.to_str().expect("invalid ID"))?,
    }
    Ok(())
}

fn main() {
    let res = try_main();
    finish_drawing().expect("Failed to reset terminal");
    if let Err(e) = res {
        eprintln!("Error: {}", e);
    }
}
