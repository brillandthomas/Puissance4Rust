mod game_logic;

fn display_board(connect4: game_logic::Connect4) -> () {
    println!("Here is the board : \n{}", connect4);
}

fn main() {
    let mut new_connect4 = game_logic::init_connect4();

    new_connect4.play_move(3);
    new_connect4.play_move(4);

    display_board(new_connect4);
}