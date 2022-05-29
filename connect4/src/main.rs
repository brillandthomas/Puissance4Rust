mod game_logic;

fn display_board(connect4: game_logic::Connect4) -> () {
    println!("Here is the board : \n{}", connect4);
}

fn main() {
    let mut new_connect4 = game_logic::init_connect4();

    for _ in 0..15 {
        new_connect4.play_random_move();
    }

    new_connect4.save_moves(String::from("Red player"), String::from("Yellow player"));
}
