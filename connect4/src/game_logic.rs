use std::ops::{Index, IndexMut};

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Player {
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
enum Cell {
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
struct Connect4 {
    board: [Cell; BOARD_WIDTH * BOARD_HEIGHT],
    columns_height: [usize; BOARD_WIDTH],
    to_play: Player,
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
        while (i < BOARD_WIDTH) & (j < BOARD_HEIGHT) {
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

    fn check_winner(&self, (row, column): (usize, usize)) -> bool {
        let other_player: Player = self.to_play.other();

        let row_list = self.get_row(row);
        let row_winner = self.check_winner_list(other_player, row_list);

        let column_list = self.get_column(column);
        let column_winner = self.check_winner_list(other_player, column_list);

        let ascending_diagonal_list = self.get_ascending_diagonal((row, column));
        let ascending_diagonal_winner =
            self.check_winner_list(other_player, ascending_diagonal_list);

        let descending_diagonal_list = self.get_descending_diagonal((row, column));
        let descending_diagonal_winner =
            self.check_winner_list(other_player, descending_diagonal_list);

        (row_winner | column_winner | ascending_diagonal_winner | descending_diagonal_winner)
    }

    fn valid_action(&self, column: usize) -> bool {
        (column < BOARD_WIDTH) & (self.columns_height[column] < BOARD_HEIGHT)
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
