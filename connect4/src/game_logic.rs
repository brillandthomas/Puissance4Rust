use rand::Rng;
use std::{
    char, cmp, fmt, fs,
    ops::{Index, IndexMut},
};

pub const BOARD_HEIGHT: usize = 6;
pub const BOARD_WIDTH: usize = 7;
pub const ALIGN_TARGET: i32 = 4;
const MAX_CHECK_LEN: usize = if BOARD_HEIGHT > BOARD_WIDTH {
    BOARD_HEIGHT
} else {
    BOARD_WIDTH
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Player {
    Red,
    Yellow,
}

impl Player {
    pub fn other(self) -> Self {
        use Player::*;
        match self {
            Red => Yellow,
            Yellow => Red,
        }
    }

    pub fn select<T>(self, player_1: T, player_2: T) -> (T, T) {
        use Player::*;
        match self {
            Red => (player_1, player_2),
            Yellow => (player_2, player_1),
        }
    }
}

impl From<Player> for char {
    fn from(player: Player) -> Self {
        Cell::from(player).into()
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Player::*;
        match self {
            Red => write!(f, "red"),
            Yellow => write!(f, "yellow"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Red,
    Yellow,
}

type CheckSlice = [Cell; MAX_CHECK_LEN];

impl From<Player> for Cell {
    fn from(player: Player) -> Self {
        match player {
            Player::Red => Cell::Red,
            Player::Yellow => Cell::Yellow,
        }
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
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
    pub to_play: Player,
    last_move: (usize, usize),
    played_moves: Vec<usize>,
}

impl Connect4 {
    pub fn check_winner(&self) -> Option<Player> {
        let player = self.to_play.other();
        let pos = self.last_move;
        let winner = [(1, 0), (0, 1), (1, 1), (1, -1)]
            .iter()
            .map(|&direction| self.sub_board(pos, direction))
            .any(|check_slice| Self::check_winner_list(player, check_slice));
        if winner {
            Some(player)
        } else {
            None
        }
    }

    pub fn new() -> Self {
        Self {
            board: [Cell::Empty; BOARD_HEIGHT * BOARD_WIDTH],
            columns_height: [0; BOARD_WIDTH],
            to_play: Player::Red,
            last_move: (0, 0),
            played_moves: Vec::new(),
        }
    }

    pub fn over(&self) -> bool {
        matches!(self.check_winner(), Some(_))
            | self
                .columns_height
                .iter()
                .all(|&height| height == BOARD_HEIGHT)
    }

    pub fn play(&mut self, column: usize) {
        let row_move = self.columns_height[column];
        self[(row_move, column)] = self.to_play.into();
        self.to_play = self.to_play.other();
        self.columns_height[column] += 1;
        self.last_move = (row_move, column);
        self.played_moves.push(column);
    }

    pub fn play_random_move(&mut self) -> () {
        let mut possible_moves: Vec<usize> = Vec::new();
        for column in 0..BOARD_WIDTH {
            if self.columns_height[column] < BOARD_HEIGHT {
                possible_moves.push(column);
            }
        }
        let chosen_index = rand::thread_rng().gen_range(0..possible_moves.len());
        self.play(possible_moves[chosen_index]);
    }

    pub fn save_moves(&self, red_player: String, yellow_player: String, file_name: String) -> () {
        let mut red_moves: Vec<char> = Vec::new();
        let mut yellow_moves: Vec<char> = Vec::new();

        for ind in 0..self.played_moves.len() {
            let c = char::from_digit(self.played_moves[ind] as u32, 10).unwrap();
            if ind % 2 == 0 {
                red_moves.push(c);
                red_moves.push(';');
            } else {
                yellow_moves.push(c);
                yellow_moves.push(';')
            }
        }

        let yellow_string: String = yellow_moves.iter().collect();
        let red_string: String = red_moves.iter().collect();

        let mut result_string: String = String::from("Not finished");

        if self.over() {
            let winner = self.check_winner();
            if winner == Some(Player::Yellow) {
                result_string = format!("Winner : {} ", yellow_player);
            } else if winner == Some(Player::Red) {
                result_string = format!("Winner : {}", red_player);
            } else {
                result_string = String::from("Draw");
            }
        }

        let total_string: String = format!(
            "{0} moves (X): {1} \n{2} moves (O): {3}\n{4}\n\nFinal board :\n{5}",
            red_player, red_string, yellow_player, yellow_string, result_string, &self
        );

        fs::write(file_name, total_string).expect("Unable to write data");
    }

    pub fn valid_action(&self, column: usize) -> bool {
        (column < BOARD_WIDTH) & (self.columns_height[column] < BOARD_HEIGHT)
    }

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
                BOARD_WIDTH, column
            );
        }
    }

    fn check_winner_list(player: Player, list: CheckSlice) -> bool {
        let mut count = 0;
        let player = Cell::from(player);
        for &played in list.iter() {
            if played == player {
                count += 1;
                if count == ALIGN_TARGET {
                    return true;
                }
            } else {
                count = 0;
            }
        }
        false
    }

    fn compute_indices((row, column): (i32, i32), (dx, dy): (i32, i32)) -> (usize, i32, i32) {
        match (dx, dy) {
            (1, 0) => (BOARD_WIDTH, row, 0),
            (0, 1) => (BOARD_HEIGHT, 0, column),
            _ => {
                let (pos_row, neg_row) = if dy == 1 {
                    (row, BOARD_HEIGHT as i32 - 1 - row)
                } else {
                    (BOARD_HEIGHT as i32 - 1 - row, row)
                };
                let mini = cmp::min(pos_row, column);
                let maxi = cmp::min(neg_row, BOARD_WIDTH as i32 - 1 - column);
                (
                    1 + (mini + maxi) as usize,
                    row as i32 - dy * mini as i32,
                    (column - mini) as i32,
                )
            }
        }
    }

    fn sub_board(&self, (row, column): (usize, usize), (dx, dy): (i32, i32)) -> CheckSlice {
        let (len, mut i, mut j) = Self::compute_indices((row as i32, column as i32), (dx, dy));
        let mut check_slice = [Cell::Empty; MAX_CHECK_LEN];
        for ind in 0..len {
            check_slice[ind] = self[(i as usize, j as usize)];
            i += dy;
            j += dx;
        }
        check_slice
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

impl fmt::Display for Connect4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_vec: Vec<char> = Vec::new();
        for row_number in (0..BOARD_HEIGHT).rev() {
            for column_number in 0..(BOARD_WIDTH) {
                board_vec.push('|');
                board_vec.push(self[(row_number, column_number)].into());
                board_vec.push('|');
            }
            board_vec.push('\n')
        }
        for column_number in 0..(BOARD_WIDTH) {
            board_vec.push(' ');
            board_vec.push(char::from_digit(column_number as u32, 36).unwrap());
            board_vec.push(' ');
        }
        let board_print: String = board_vec.iter().collect();
        write!(f, "{}", board_print)
    }
}
