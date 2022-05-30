use rand::Rng;
use std::{
    char, cmp, fmt, fs,
    ops::{Index, IndexMut},
};

//Size of the Connect4 board
pub const BOARD_HEIGHT: usize = 6;
pub const BOARD_WIDTH: usize = 7;
pub const ALIGN_TARGET: i32 = 4;
const MAX_CHECK_LEN: usize = if BOARD_HEIGHT > BOARD_WIDTH {
    BOARD_HEIGHT
} else {
    BOARD_WIDTH
};

// Player enumeration and helpful functions
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Player {
    Red,
    Yellow,
}

impl Player {
    pub fn other(self) -> Self {
        // Returns the other player
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

// Enumeration for each cell, empty / red / yellow
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Red,
    Yellow,
}

type CheckSlice = [Cell; MAX_CHECK_LEN];

// Get cell from player
impl From<Player> for Cell {
    fn from(player: Player) -> Self {
        match player {
            Player::Red => Cell::Red,
            Player::Yellow => Cell::Yellow,
        }
    }
}

// Get character from cell
impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Empty => ' ',
            Cell::Red => 'X',
            Cell::Yellow => 'O',
        }
    }
}

// Implementation of the connect4 game
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Connect4 {
    pub board: [Cell; BOARD_WIDTH * BOARD_HEIGHT], // board from bottom to top
    columns_height: [usize; BOARD_WIDTH], //for each column, number of nonempty cells
    pub to_play: Player, // player that has to play next turn
    last_move: (usize, usize), // last move that was played
    played_moves: Vec<usize>, // list of the played moves
}

impl Connect4 {
    // check if the board is fully completed
    pub fn check_full(&self) -> bool {
        self.columns_height
            .iter()
            .all(|&height| height == BOARD_HEIGHT)
    }

    // check if last move was a winning move
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

    // Create an empty board to initialize a game
    pub fn new() -> Self {
        Self {
            board: [Cell::Empty; BOARD_HEIGHT * BOARD_WIDTH],
            columns_height: [0; BOARD_WIDTH],
            to_play: Player::Red,
            last_move: (0, 0),
            played_moves: Vec::new(),
        }
    }

    // Check if the game is over (draw, full or win)
    pub fn over(&self) -> bool {
        matches!(self.check_winner(), Some(_)) | self.check_full()
    }

    // Play a move in the column input (consider that the column is valid)
    pub fn play(&mut self, column: usize) {
        let row_move = self.columns_height[column];
        self[(row_move, column)] = self.to_play.into();
        self.to_play = self.to_play.other();
        self.columns_height[column] += 1;
        self.last_move = (row_move, column);
        self.played_moves.push(column);
    }

    // Play a random possible move
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

    // Save a file for a recap of the game
    pub fn save(&self, filename: String, history: Vec<u8>) -> () {
        // Played moves list for each player
        let mut red_moves: Vec<char> = Vec::new();
        let mut yellow_moves: Vec<char> = Vec::new();
    
        // Fill the moves list for each player
        for (num_action, &action) in history.iter().enumerate() {
            let action = char::from_digit(action as u32, 10).unwrap();
            let buffer = if num_action % 2 == 0 {
                &mut red_moves
            } else {
                &mut yellow_moves
            };
            buffer.push(action);
            buffer.push(';');
        }

        // Convert the list of moves into a string
        let yellow_string: String = yellow_moves.iter().collect();
        let red_string: String = red_moves.iter().collect();
        let result_string = if self.over() {
            if let Some(winner) = self.check_winner() {
                format!("Winner : {}", winner)
            } else {
                String::from("Draw")
            }
        } else {
            String::from("Not finished")
        };
        // Final string we will save in a file.
        let total_string: String = format!(
            "Red moves ({}): {} \nYellow moves ({}): {}\n{}\n\nFinal board :\n{}",
            char::from(Player::Red),
            red_string,
            char::from(Player::Yellow),
            yellow_string,
            result_string,
            self
        );
        fs::write(filename, total_string).expect("Unable to write data.");
    }

    // Verify if an action is valid (column in the board + the column is not full)
    pub fn valid_action(&self, column: usize) -> bool {
        (column < BOARD_WIDTH) & (self.columns_height[column] < BOARD_HEIGHT)
    }

    // Check if the coordinates are inside the board
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

    // Verify if there is 4 Cells aligned in a list
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

    // Get the slices for the row, the column or the diagonals for the last move
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

    // Get the vector
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

// Index for Connect4 board using directly the row and column numbers
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

// Display the connect4 board
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
