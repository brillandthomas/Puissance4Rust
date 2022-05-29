use std::ops::{Index, IndexMut};

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Player {
    Red,
    Yellow,
}

impl Player {
    fn other(self) -> Self {
        use Player::*;
        match self {
            Red => Yellow,
            Yellow => Red,
        }
    }

    pub fn select<T>(self, player_1: T, player_2: T) -> (T, T) {
        match self {
            Red => (player_1, player_2),
            Yellow => (player_2, player_1),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Red,
    Yellow,
}

impl Into<Cell> for Player {
    fn into(self) -> Cell {
        match self {
            Player::Red => Cell::Red,
            Player::Yellow => Cell::Yellow,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Connect4 {
    board: [Cell; BOARD_WIDTH * BOARD_HEIGHT],
    columns_height: [usize; BOARD_WIDTH],
    pub to_play: Player,
}

impl Connect4 {
    fn check_coordinates(row: usize, column: usize) {
        if row >= BOARD_HEIGHT {
            panic!(
                "row out of bounds: the height is {} but the row is {}",
                BOARD_HEIGHT, row
            );
        }
        if column >= BOARD_WIDTH {
            panic!(
                "column out of bounds: the width is {} but the column is {}",
                BOARD_WIDTH, row
            );
        }
    }

    pub fn has_winner(&self) -> Option<Player> {
        None
    }

    pub fn new() -> Self {
        Connect4 {
            board: [Cell::Empty; BOARD_HEIGHT * BOARD_WIDTH],
            columns_height: [0; BOARD_WIDTH],
            to_play: Player::Red,
        }
    }

    pub fn over(&self) -> bool {
        false
    }

    pub fn valid_action(&self, column: usize) -> bool {
        self.columns_height[column] < BOARD_HEIGHT
    }

    pub fn play(&mut self, _column: usize) {
        self[(0, 0)] = self.to_play.into()
    }
}

impl Index<(usize, usize)> for Connect4 {
    type Output = Cell;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        Self::check_coordinates(row, column);
        let index = BOARD_WIDTH * row + column;
        unsafe { self.board.get_unchecked(index) }
    }
}

impl IndexMut<(usize, usize)> for Connect4 {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        Self::check_coordinates(row, column);
        let index = BOARD_WIDTH * row + column;
        unsafe { self.board.get_unchecked_mut(index) }
    }
}
