use derive_more::{Add, AddAssign};

#[derive(Clone, Copy)]
pub enum ScorchState {
    Empty,
    Scorched,
}

#[derive(Clone, Copy)]
pub enum TileContents {
    Empty,
    Scorched,
    Player(PlayerColor),
}

impl From<ScorchState> for TileContents {
    fn from(value: ScorchState) -> Self {
        match value {
            ScorchState::Empty => Self::Empty,
            ScorchState::Scorched => Self::Scorched,
        }
    }
}

#[derive(Clone, Copy, Add, AddAssign, PartialEq)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vector(&self) -> Vector {
        match self {
            Direction::Up => Vector { x: 0, y: -1 },
            Direction::Down => Vector { x: 0, y: 1 },
            Direction::Left => Vector { x: -1, y: 0 },
            Direction::Right => Vector { x: 1, y: 0 },
        }
    }
}

#[derive(Clone, Copy)]
pub enum PlayerColor {
    Blue,
    Cyan,
    White,
    Yellow,
    Green,
    Magenta,
}

pub struct Player {
    pub pos: Vector,
    pub color: PlayerColor,
}

pub struct TurnResult {
    pub winner: Option<PlayerColor>,
    pub changes: Vec<(Vector, TileContents)>,
}

pub struct Board<const N: usize> {
    pub cells: [[ScorchState; N]; N],
    pub players: Vec<Player>,
}

impl<const N: usize> Board<N> {
    // Get the state of the tile at this position, or return None if there isn't one there
    pub fn scorch_state_at(&self, pos: Vector) -> Option<&ScorchState> {
        let Ok(x): Result<usize, _> = pos.x.try_into() else { return None };
        let Ok(y): Result<usize, _> = pos.y.try_into() else { return None };

        self.cells.get(y).map(|row| row.get(x)).flatten()
    }

    // Same as tile_at but mutable so it can be changed
    pub fn scorch_state_at_mut(&mut self, pos: Vector) -> Option<&mut ScorchState> {
        let Ok(x): Result<usize, _> = pos.x.try_into() else { return None };
        let Ok(y): Result<usize, _> = pos.y.try_into() else { return None };

        self.cells.get_mut(y).map(|row| row.get_mut(x)).flatten()
    }

    pub fn tile_contents_at(&self, pos: Vector) -> Option<TileContents> {
        for player in &self.players {
            if player.pos == pos {
                return Some(TileContents::Player(player.color));
            }
        }

        self.scorch_state_at(pos).copied().map(|scorch_state| scorch_state.into())
    }

    // Move the specified player in the specified position
    // Returns None and does nothing if the move is impossible, so they can try again
    pub fn try_move(&mut self, player_index: usize, direction: Direction) -> Option<TurnResult> {
        let target_pos = self.players[player_index].pos + direction.to_vector();
        match self.scorch_state_at(target_pos) {
            Some(ScorchState::Empty) => {
                let current_pos = self.players[player_index].pos;
                let current_cell = self
                    .scorch_state_at_mut(current_pos)
                    .expect("Invalid player position for move");
                *current_cell = ScorchState::Scorched;

                self.players[player_index].pos += direction.to_vector();


                let mut changes = vec![
                    (current_pos, TileContents::Scorched),
                    (
                        self.players[player_index].pos,
                        TileContents::Player(self.players[player_index].color),
                    ),
                ];

                // Remove players that lost (in reverse order to avoid messing up the indices
                // during the loop)
                for i in (0..self.players.len()).rev() {
                    if self.player_lost(i) {
                        let removed_position = self.players[i].pos;
                        if let Some(contents) = self.tile_contents_at(removed_position) {
                            changes.push((removed_position, contents));
                        }
                        self.players.remove(i);
                    }
                }

                let winner = if self.players.len() == 1 {
                    Some(self.players[0].color)
                } else {
                    None
                };

                Some(TurnResult { winner, changes })
            }
            _ => None,
        }
    }

    // Check if the specified player has lost, due to checkmate or capture
    pub fn player_lost(&mut self, player_index: usize) -> bool {
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
        .map(|direction| self.scorch_state_at(self.players[player_index].pos + direction.to_vector()))
        .flatten()
        .all(|cell| matches!(cell, ScorchState::Scorched))
    }
}

impl<const N: usize> Default for Board<N> {
    // Make a default board that's empty with a player in opposite corners
    fn default() -> Self {
        Board {
            cells: [[ScorchState::Empty; N]; N],
            players: vec![
                Player {
                    pos: Vector { x: 0, y: 0 },
                    color: PlayerColor::Green,
                },
                Player {
                    pos: Vector {
                        x: (N - 1) as isize,
                        y: (N - 1) as isize,
                    },
                    color: PlayerColor::Yellow,
                },
            ],
        }
    }
}
