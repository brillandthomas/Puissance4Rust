use crate::{
    communication::Message::{self, *},
    game_logic::{self, Connect4, Player},
};
use std::{io, net::TcpStream};

// Run the client
pub fn run(socket_address: (String, u16), replay_file: Option<String>) {
    let mut server = TcpStream::connect(socket_address).unwrap();
    let color = match Message::receive_from(&mut server) {
        Hello(color) => color,
        message => panic!("Unexpected message: {:?}", message),
    };
    let (game, game_history) = play_game(server, color);
    if let Some(filename) = replay_file {
        game.save(filename, game_history);
    }
}

// Send the game over message
fn game_over(result: Message) {
    match result {
        Lose => println!("You lost the game."),
        Draw => println!("The game ended in a draw."),
        Win => println!("Congratulations, you won the game!"),
        _ => unreachable!(),
    }
}

// Get the input from the player in the client
fn input_action(server: &mut TcpStream) {
    loop {
        println!("\nPlease input your move:");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read action.");
        let action: u8 = match action.trim().parse() {
            Ok(column) => column,
            Err(_) => continue,
        };
        Action(action).send_to(server);
        break;
    }
}

// Function for playing the game using the server
fn play_game(mut server: TcpStream, color: Player) -> (Connect4, Vec<u8>) {
    let mut game = Connect4::new();
    println!(
        "You are playing with {} (symbol: {}).\n Columns are numbered from 0 to {} inclusive, \
            starting from the left.\n\n{}\n\n",
        color,
        char::from(color),
        game_logic::BOARD_WIDTH - 1,
        game,
    );
    let mut game_history = Vec::new();
    loop {
        match Message::receive_from(&mut server) {
            Play => input_action(&mut server),
            InvalidAction => {
                println!("\nInvalid action.");
            }
            ValidAction(action) => {
                game.play(action as usize);
                game_history.push(action);
                println!(
                    "\n{}\n\nA token has been placed in column {}.\n",
                    game, action
                );
            }
            result @ (Lose | Draw | Win) => {
                game_over(result);
                return (game, game_history);
            }
            message => panic!("Unexpected message: {:?}", message),
        };
    }
}
