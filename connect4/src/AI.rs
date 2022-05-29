
use game_logic;

pub fn chose_move_AI(connect4_game: game_logic::Connect4) -> usize {

    let mut winning_move : usize = 0;
    let mut result_winner = None;

    while winning_move < game_logic::BOARD_WIDTH {
        let future_game = connect4_game.clone();
        if future_game.valid_action(winning_move) {
            future_game.play_move(winning_move);
            result_winner = future_game.check_winner();
        }
        if result_winner != None {
            return winning_move
        }
        winning_move = winning_move + 1;
    }

    let mut losing_move : usize = 0;

    while losing_move < game_logic::BOARD_WIDTH {
        let future_game = connect4_game.clone();
        future_game.to_play = future_game.to_play.other_player();
        if future_game.valid_action(losing_move) {
            future_game.play_move(losing_move);
            result_winner = future_game.check_winner();
        }
        if result_winner != None {
            return losing_move
        }
        losing_move = losing_move + 1
    }

    let priority_moves : Vec<usize> = [3,4,2,5,1,6,0];
    let mut chosen_index = 0;
    let mut chosen_move = priority_moves[chosen_index];

    while connect4_game.valid_action(chosen_move) == false {
        chosen_index = chosen_index + 1;
        chosen_move = priority_moves[chosen_index];
    }

    chosen_move
}