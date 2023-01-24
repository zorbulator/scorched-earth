use anyhow::Result;
use crossterm::{
    cursor::{
        Hide, MoveDown, MoveLeft, MoveRight, MoveUp, RestorePosition, SavePosition,
        Show,
    },
    event::{Event, KeyCode, KeyEvent},
    execute,
    style::{Color, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use derive_more::{Add, AddAssign};
use std::io::{stdout, Write};

// Width/height of the board
const BOARD_SIZE: usize = 10;

#[derive(Clone, Copy)]
enum CellState {
    Empty,
    Scorched,
}

#[derive(Clone, Copy, Add, AddAssign, PartialEq)]
struct Vector {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_vector(&self) -> Vector {
        match self {
            Direction::Up => Vector { x: 0, y: -1 },
            Direction::Down => Vector { x: 0, y: 1 },
            Direction::Left => Vector { x: -1, y: 0 },
            Direction::Right => Vector { x: 1, y: 0 },
        }
    }
}

struct Player {
    pos: Vector,
    color: Color,
}

struct Board {
    cells: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    players: Vec<Player>,
}

impl Board {
    // Initial setup for drawing moves
    fn setup_drawing(&self) -> crossterm::Result<()> {
        // Disables typing to the terminal so keyboard input isn't visible
        enable_raw_mode()?;
        print!("\n");

        // Hide the cursor
        execute!(stdout(), Hide,)?;

        // Draw the board with a grey border and blank inside
        for i in 0..BOARD_SIZE + 2 {
            for j in 0..BOARD_SIZE + 2 {
                let color = if i == 0 || j == 0 || i == BOARD_SIZE + 1 || j == BOARD_SIZE + 1 {
                    Color::Grey
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
        for player in &self.players {
            Board::draw_cell(player.pos, player.color)?;
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
    fn draw_cell(pos: Vector, color: Color) -> crossterm::Result<()> {
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

    // Get the state of the cell at this position, or return None if there isn't one there
    fn cell_at(&self, pos: Vector) -> Option<&CellState> {
        let Ok(x): Result<usize, _> = pos.x.try_into() else { return None };
        let Ok(y): Result<usize, _> = pos.y.try_into() else { return None };

        self.cells.get(y).map(|row| row.get(x)).flatten()
    }

    // Same as cell_at but mutable so it can be changed
    fn cell_at_mut(&mut self, pos: Vector) -> Option<&mut CellState> {
        let Ok(x): Result<usize, _> = pos.x.try_into() else { return None };
        let Ok(y): Result<usize, _> = pos.y.try_into() else { return None };

        self.cells.get_mut(y).map(|row| row.get_mut(x)).flatten()
    }

    // Move the specified player in the specified position
    // Returns None and does nothing if the move is impossible, so they can try again
    fn try_move(&mut self, player_index: usize, direction: Direction) -> Option<()> {
        let target_pos = self.players[player_index].pos + direction.to_vector();
        match self.cell_at(target_pos) {
            Some(CellState::Empty) => {
                let current_pos = self.players[player_index].pos;
                let current_cell = self
                    .cell_at_mut(current_pos)
                    .expect("Invalid player position for move");
                *current_cell = CellState::Scorched;
                Board::draw_cell(self.players[player_index].pos, Color::Red)
                    .expect("Couldn't draw scorched tile!");

                self.players[player_index].pos += direction.to_vector();

                Board::draw_cell(
                    self.players[player_index].pos,
                    self.players[player_index].color,
                )
                .expect("Couldn't draw moved player!");
            }
            _ => return None,
        }

        Some(())
    }


    // Check if the specified player has lost, due to checkmate or capture
    fn player_lost(&mut self, player_index: usize) -> bool {
        for (i, player) in self.players.iter().enumerate() {
            if i != player_index {
                if player.pos == self.players[player_index].pos {
                    return true;
                }
            }
        }

        [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
        .iter()
        .map(|direction| self.cell_at(self.players[player_index].pos + direction.to_vector()))
        .flatten()
        .all(|cell| matches!(cell, CellState::Scorched))
    }
}

impl Default for Board {
    // Make a default board that's empty with a player in opposite corners
    fn default() -> Self {
        Board {
            cells: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            players: vec![
                Player {
                    pos: Vector { x: 0, y: 0 },
                    color: Color::Green,
                },
                Player {
                    pos: Vector {
                        x: (BOARD_SIZE - 1) as isize,
                        y: (BOARD_SIZE - 1) as isize,
                    },
                    color: Color::Yellow,
                },
            ],
        }
    }
}

fn main() -> Result<()> {
    let mut b = Board::default();
    b.setup_drawing()?;

    'main: loop {
        // Give each player a turn in order until the main loop is over
        for i in 0..b.players.len() {
            if b.player_lost(i) {
                // If the current player lost, remove them
                b.players.remove(i);

                if b.players.len() == 1 {
                    // If there's only one left (always the case with the default 2 players), they
                    // win so fill in the board with their color
                    Board::fill_box(b.players[0].color)?;
                    break 'main;
                } else {
                    // If there are more players left skip the losing player's turn and keep going
                    continue;
                }
            }

            // Set the border to show the current player's color
            Board::draw_border(b.players[i].color)?;
            // loop until a valid move is made
            loop {
                match crossterm::event::read()? {
                    // Wait for a keypress and only accept it if it's wasd or q
                    Event::Key(KeyEvent {
                        code: KeyCode::Char(c @ ('w' | 'a' | 's' | 'd' | 'q')),
                        ..
                    }) => {
                        // Quit on q, otherwise get the direction that was pressed
                        let dir = match c {
                            'q' => break 'main,
                            'w' => Direction::Up,
                            'a' => Direction::Left,
                            's' => Direction::Down,
                            'd' => Direction::Right,
                            _ => unreachable!(),
                        };


                        // Try to actually make the move, and try again if it's invalid
                        match b.try_move(i, dir) {
                            Some(()) => {
                                break;
                            }
                            None => {}
                        }
                    }

                    _ => {}
                };
            }
        }
    }

    Board::finish_drawing()?;
    Ok(())
}
