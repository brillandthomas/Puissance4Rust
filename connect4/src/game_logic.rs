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
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Red,
    Yellow,
}

impl Cell {
    fn is_empty(self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    } 
}


impl Into<Cell> for Player {
    fn into(self) -> Cell {
        match self {
            Player::Red => Cell::Red,
            Player::Yellow => Cell::Yellow,
        }
    }
}

impl Into<char> for Cell {
    fn into(self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::Red => 'X',
            Cell::Yellow => 'O',
        }
    }
} 

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Connect4 {
    board: [Cell; BOARD_WIDTH * BOARD_HEIGHT],
    columns_height: [usize; BOARD_WIDTH],
    to_play: Player,
    last_move: (usize, usize),
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

    fn get_row(&self, row: usize) -> [Cell; BOARD_WIDTH] {
        self.board[BOARD_WIDTH * row..BOARD_WIDTH * (row + 1)]
            .try_into()
            .unwrap()
    }

    fn get_column(&self, column: usize) -> [Cell; BOARD_WIDTH] {
        let mut column_output = [Cell::Empty; BOARD_WIDTH];
        for x in 0..BOARD_HEIGHT {
            column_output[x] = self[(x, column)];
        }
        column_output
    }

    fn get_ascending_diagonal(&self, (row, column): (usize, usize)) -> [Cell; BOARD_WIDTH] {
        let mut diagonal_output = [Cell::Empty; BOARD_WIDTH];
        let mut i = row;
        let mut j = column;
        let mut ind = 0;
        while (i > 0) & (j > 0) {
            i = i - 1;
            j = j - 1;
        }
        while (i < BOARD_HEIGHT) & (j < BOARD_WIDTH) {
            diagonal_output[ind] = self[(i, j)];
            ind = ind + 1;
            i = i + 1;
            j = j + 1;
        }
        diagonal_output
    }

    fn get_descending_diagonal(&self, (row, column): (usize, usize)) -> [Cell; BOARD_WIDTH] {
        let mut diagonal_output = [Cell::Empty; BOARD_WIDTH];
        let mut i = row;
        let mut j = column;
        let mut ind = 0;
        while (i > 0) & (j < BOARD_WIDTH) {
            i = i - 1;
            j = j + 1;
        }
        while (i < BOARD_HEIGHT) & (j > 0) {
            diagonal_output[ind] = self[(i, j)];
            ind = ind + 1;
            i = i + 1;
            j = j - 1;
        }
        diagonal_output
    }

    fn check_winner_list(&self, player: Player, list: [Cell; BOARD_WIDTH]) -> bool {
        let mut count = 0;
        let player: Cell = player.into();
        for played in list.iter() {
            if played == &player {
                count = count + 1;
                if count == 4 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
        false
    }

    pub fn check_winner(&self) -> bool {
        let other_player: Player = self.to_play.other();
        let (row,column): (usize, usize) = self.last_move;

        let row_list = self.get_row(row);
        let row_winner = self.check_winner_list(other_player, row_list);
        if row_winner {
            return true;
        }

        let column_list = self.get_column(column);
        let column_winner = self.check_winner_list(other_player, column_list);
        if column_winner {
            return true;
        }

        let ascending_diagonal_list = self.get_ascending_diagonal((row, column));
        let ascending_diagonal_winner =
            self.check_winner_list(other_player, ascending_diagonal_list);
        if ascending_diagonal_winner {
            return true;
        }

        let descending_diagonal_list = self.get_descending_diagonal((row, column));
        let descending_diagonal_winner =
            self.check_winner_list(other_player, descending_diagonal_list);
        if descending_diagonal_winner {
            return true;
        }

        false
    }

    pub fn check_draw(&self) -> bool {
        for column in 0..BOARD_WIDTH {
            if self.columns_height[column] < BOARD_HEIGHT {
                return false;
            }
        }
        true
    }

    pub fn valid_action(&self, column: usize) -> bool {
        (column < BOARD_WIDTH) & (self.columns_height[column] < BOARD_HEIGHT)
    }

    pub fn play_move(&mut self, column: usize) -> () {
        let row_move = self.columns_height[column];
        self[(row_move, column)] = self.to_play.into();
        self.to_play = self.to_play.other();
        self.columns_height[column] = self.columns_height[column] + 1;
        self.last_move = (row_move, column);
    }
}

pub fn init_connect4() -> Connect4 {
    let new_connect4 = Connect4 {
        board : [Cell::Empty; BOARD_HEIGHT*BOARD_WIDTH],
        columns_height : [0; BOARD_WIDTH],
        to_play : Player::Red,
        last_move : (10,10),
    };
    new_connect4
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

use std::fmt;

impl fmt::Display for Connect4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut board_vec: Vec<char> = Vec::new();
        for row_number in (0..BOARD_HEIGHT).rev() {
            for column_number in 0..(BOARD_WIDTH) {
                board_vec.push('|');
                board_vec.push(self[(row_number, column_number)].into());
                board_vec.push('|');
            }
            board_vec.push('\n')
        }
        
        let board_print : String = board_vec.iter().collect(); 

        write!(f, "{}", board_print)
    }
}
