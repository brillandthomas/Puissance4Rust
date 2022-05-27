mod game_logic;

fn main() {
    let new_connect4 = game_logic::init_connect4();
    println!("{}", format!("Here is the board : \n {new_connect4}"))
}