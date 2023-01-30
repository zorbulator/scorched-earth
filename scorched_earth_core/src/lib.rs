use derive_more::{Add, AddAssign, Mul};

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

#[derive(Clone, Copy, Add, AddAssign, Mul, PartialEq)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Move {
    pub dir: Direction,
    pub len: usize,
}

impl Move {
    pub fn to_vector(&self) -> Vector {
        self.dir.to_vector() * (self.len as isize)
    }
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

    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy)]
pub enum PlayerColor {
    Blue,
    Cyan,
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

        self.scorch_state_at(pos)
            .copied()
            .map(|scorch_state| scorch_state.into())
    }

    pub fn is_move_valid(&self, player_index: usize, attempted_move: Move) -> bool {
        matches!(
            self.scorch_state_at(self.players[player_index].pos + attempted_move.to_vector()),
            Some(ScorchState::Empty)
        )
    }

    // Move the specified player with the specified move
    pub fn make_move(&mut self, player_index: usize, attempted_move: Move) -> TurnResult {
        let mut changes = Vec::new();

        for _ in 0..attempted_move.len {
            let current_pos = self.players[player_index].pos;
            let current_cell = self
                .scorch_state_at_mut(current_pos)
                .expect("Invalid player position for move");

            *current_cell = ScorchState::Scorched;
            changes.push((current_pos, TileContents::Scorched));
            self.players[player_index].pos += attempted_move.dir.to_vector();
        }

        changes.push((
            self.players[player_index].pos,
            TileContents::Player(self.players[player_index].color),
        ));

        // Remove players that lost (in reverse order to avoid messing up the indices
        // during the loop)

        // This has problems because both players can lose at once and when this happens either
        // player can win based on their order in the list when the current player should always
        // win in case of a conflict. Reordering the indices to put the current player last would also
        // cause problems because removing the current player would change the indices of
        // everything after it, which is why they have to be in reverse order.
        // This could be fixed by reordering the players list itself or not actually
        // deleting the players or keeping track of them with something other than indices.

        /*for i in (0..self.players.len()).rev() {
            if self.player_lost(i) {
                let removed_position = self.players[i].pos;
                self.players.remove(i);
                if let Some(contents) = self.tile_contents_at(removed_position) {
                    changes.push((removed_position, contents));
                }
            }
        }

        let winner = if self.players.len() == 1 {
            Some(self.players[0].color)
        } else {
            None
        };*/

        // Since games are always 2 players for now, just check each one in the correct order
        let mut winner = None;

        if self.player_lost(1 - player_index) {
            winner = Some(self.players[player_index].color);
        } else if self.player_lost(player_index) {
            winner = Some(self.players[1 - player_index].color);
        }

        TurnResult { winner, changes }
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
        .map(|direction| {
            self.scorch_state_at(self.players[player_index].pos + direction.to_vector())
        })
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
