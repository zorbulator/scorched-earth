use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveDown, MoveLeft, MoveRight, MoveUp, RestorePosition, SavePosition, Show},
    event::{Event, KeyCode, KeyEvent},
    execute,
    style::{Color, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::io::{stdout, Write};

use scorched_earth::{Board, Direction, Move, PlayerColor, TileContents, Vector};

// Width/height of the board
const BOARD_SIZE: usize = 11;

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
fn setup_drawing<const N: usize>(board: &Board<N>) -> crossterm::Result<()> {
    // Disables typing to the terminal so keyboard input isn't visible
    enable_raw_mode()?;
    print!("\n");

    // Hide the cursor
    execute!(stdout(), Hide,)?;

    // Draw the board with a grey border and blank inside
    for i in 0..N + 2 {
        for j in 0..N + 2 {
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

fn main() -> Result<()> {
    let mut b = Board::<BOARD_SIZE>::default();
    setup_drawing(&b)?;

    'main: loop {
        // Give each player a turn in order until the main loop is over
        for i in 0..b.players.len() {
            // Set the border to show the current player's color
            draw_border(player_term_color(b.players[i].color))?;

            // loop until a valid move is made
            let mut m: Option<Move> = None;
            loop {
                match crossterm::event::read()? {
                    // Wait for a keypress and only accept it if it's wasd or q
                    Event::Key(KeyEvent {
                        code: KeyCode::Char(c @ ('w' | 'a' | 's' | 'd' | 'q' | ' ')),
                        ..
                    }) => {
                        // If there was previously a move preview, re-render that tile so it goes
                        // away
                        if let Some(potential_move) = m {
                            let target_position = b.players[i].pos + potential_move.to_vector();
                            if let Some(contents) = b.tile_contents_at(target_position) {
                                draw_tile_contents(target_position, contents)?;
                            }
                        }

                        // Quit on q, otherwise get the direction that was pressed
                        let input_dir = match c {
                            'q' => break 'main,
                            'w' => Direction::Up,
                            'a' => Direction::Left,
                            's' => Direction::Down,
                            'd' => Direction::Right,
                            ' ' => {
                                if let Some(valid_move) =
                                    m.filter(|potential_move| b.is_move_valid(i, *potential_move))
                                {
                                    let res = b.make_move(i, valid_move);
                                    for (pos, contents) in res.changes {
                                        draw_tile_contents(pos, contents)?;
                                    }

                                    if let Some(color) = res.winner {
                                        fill_box(player_term_color(color))?;
                                        break 'main;
                                    }

                                    break;
                                } else {
                                    continue;
                                }
                            }
                            _ => unreachable!(),
                        };

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
                                draw_tile(target_position, color)?;
                            }
                        }
                    }

                    _ => {}
                };
            }
        }
    }

    finish_drawing()?;
    Ok(())
}
