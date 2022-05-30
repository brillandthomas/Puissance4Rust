use crate::{
    ai,
    communication::Message::{self, *},
    game_logic::Connect4,
};
use std::net::TcpStream;

// Run the client for the ai
pub fn run(socket_address: (String, u16), depth: usize) {
    let mut server = TcpStream::connect(socket_address).unwrap();
    Message::receive_from(&mut server);
    let mut game = Connect4::new();
    loop {
        match Message::receive_from(&mut server) {
            Play => {
                let action = ai::action(game.clone(), depth);
                Action(action as u8).send_to(&mut server);
            }
            ValidAction(action) => game.play(action as usize),
            Lose | Draw | Win => break,
            message => panic!("Unexpected message: {:?}", message),
        }
    }
}
