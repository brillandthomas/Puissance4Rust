use crate::game_logic::{Cell, Connect4, BOARD_HEIGHT, BOARD_WIDTH};
use std::{
    sync::{Arc, Mutex},
    thread,
};

// Conditionnal compilation : different default depth value when compiling in debug or release mode
#[cfg(debug_assertions)]
pub const DEFAULT_DEPTH: &'static str = "8";

#[cfg(not(debug_assertions))]
pub const DEFAULT_DEPTH: &'static str = "9";

// Score values for each cell
const VALUES: [i32; BOARD_WIDTH * BOARD_HEIGHT] = [
    3, 6, 10, 15, 10, 6, 3, 4, 7, 12, 17, 12, 7, 4, 5, 8, 15, 22, 15, 8, 5, 4, 8, 14, 19, 14, 8, 4,
    3, 7, 11, 16, 11, 7, 3, 2, 5, 9, 12, 9, 5, 2,
];

// Priority moves in order, from the center to the sides
const PRIORITY_MOVES: [usize; BOARD_WIDTH] = [3, 2, 4, 1, 5, 0, 6];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MinMax {
    Min,
    Max,
}

use MinMax::*;

impl MinMax {
    #[inline]
    fn compare(self, x: i32, y: i32) -> bool {
        match self {
            Min => x > y,
            Max => x < y,
        }
    }

    #[inline]
    fn default_value(self) -> i32 {
        match self {
            Min => i32::MAX,
            Max => -i32::MAX,
        }
    }

    #[inline]
    fn other(self) -> Self {
        match self {
            Min => Max,
            Max => Min,
        }
    }
}

// wrapper for (alpha, beta)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct AlphaBeta(i32, i32);

type SharedAlphaBeta = Arc<Mutex<AlphaBeta>>;

impl AlphaBeta {
    #[inline]
    fn update(&mut self, value: i32, min_max: MinMax) -> bool {
        match min_max {
            Min => {
                if self.0 >= value {
                    true
                } else {
                    if self.1 < value {
                        self.1 = value;
                    }
                    false
                }
            }
            Max => {
                if self.1 <= value {
                    true
                } else {
                    if self.0 > value {
                        self.0 = value;
                    }
                    false
                }
            }
        }
    }
}

// chose the best move with a multi-threaded minimax algorithm with alpha-beta pruning
pub fn ai_action(game: Connect4, depth: usize) -> usize {
    let alpha_beta = AlphaBeta(-i32::MAX, i32::MAX);
    let alpha_beta = Arc::new(Mutex::new(alpha_beta));
    let mut handles = Vec::with_capacity(BOARD_WIDTH);
    for action in 0..BOARD_WIDTH {
        if game.valid_action(action) {
            let mut game = game.clone();
            game.play(action);
            let alpha_beta = Arc::clone(&alpha_beta);
            let handle = thread::spawn(move || alpha_beta_search(game, depth - 1, alpha_beta, Min));
            handles.push((handle, action));
        }
    }
    handles
        .into_iter()
        .map(|(handle, action)| (handle.join().unwrap(), action))
        .max()
        .unwrap()
        .1
}

// recursive procedure for the minimax algorithm with alpha-beta pruning
fn alpha_beta_search(
    game: Connect4,
    depth: usize,
    alpha_beta: SharedAlphaBeta,
    min_max: MinMax,
) -> i32 {
    if let Some(_) = game.check_winner() {
        return min_max.default_value();
    }
    if game.check_full() {
        return 0;
    }
    if depth == 0 {
        return evaluate(game);
    }
    let mut best = min_max.default_value();
    for &action in PRIORITY_MOVES.iter() {
        if game.valid_action(action) {
            let mut game = game.clone();
            game.play(action);
            let score =
                alpha_beta_search(game, depth - 1, Arc::clone(&alpha_beta), min_max.other());
            if min_max.compare(best, score) {
                let mut alpha_beta = alpha_beta.lock().unwrap();
                if alpha_beta.update(score, min_max) {
                    return score;
                }
                best = score;
            }
        }
    }
    best
}

// Evaluate a board using the VALUES. This is a simplistic evaluation function. Our aim was to
// implement a multi-threaded minimax algorithm with alpha-beta pruning, not to make a
// high-performance ai.
#[inline]
fn evaluate(game: Connect4) -> i32 {
    let player = Cell::from(game.to_play.other());
    let mut score = 0;
    for (&val, &cell) in VALUES.iter().zip(game.board.iter()) {
        if cell == player {
            score += val;
        } else {
            score -= val;
        }
    }
    score
}
