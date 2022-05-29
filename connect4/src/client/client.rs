use crate::{
    communication::Message::{self, *},
    game_logic::{self, Connect4, Player},
};
use std::{io, net::TcpStream};

pub fn run(socket_address: (String, u16)) {
    let mut server = TcpStream::connect(socket_address).unwrap();
    let color = match Message::receive_from(&mut server) {
        Hello(color) => color,
        message => panic!("Unexpected message: {:?}", message),
    };
    play_game(server, color);
}

fn game_over(result: Message) {
    match result {
        Lose => println!("You lost the game."),
        Draw => println!("The game ended in a draw."),
        Win => println!("Congratulations, you won the game!"),
        _ => unreachable!(),
    }
}

fn input_action(server: &mut TcpStream) {
    loop {
        print!("\nPlease input your move:\t");
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

fn play_game(mut server: TcpStream, color: Player) {
    let mut game = Connect4::new();
    println!(
        "You are playing with {} (symbol: {}).\n Columns are numbered from 0 to {} inclusive, \
            starting from the left.\n",
        color,
        char::from(color),
        game_logic::BOARD_WIDTH - 1
    );

    loop {
        match Message::receive_from(&mut server) {
            Play => input_action(&mut server),
            InvalidAction => {
                println!("\nInvalid action.");
                input_action(&mut server);
            }
            ValidAction(action) => {
                game.play(action as usize);
                println!(
                    "\n{}\n\nA token has been placed in column {}.\n",
                    game, action
                );
            }
            result @ (Lose | Draw | Win) => {
                game_over(result);
                break;
            }
            message => panic!("Unexpected message: {:?}", message),
        };
    }
}
