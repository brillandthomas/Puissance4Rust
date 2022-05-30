use crate::{
    ai, client,
    communication::Message::{self, *},
    game_logic::Connect4,
    server,
};
use rand::Rng;
use std::{
    marker,
    net::TcpStream,
    thread::{self, JoinHandle},
    time,
};

pub fn play_against(socket_address: (String, u16), depth: usize, save_replay: Option<String>) {
    let address = socket_address.clone();
    thread::spawn(|| server::run(address));
    thread::sleep(time::Duration::from_millis(500));
    let address = socket_address.clone();
    let run_client = || client::run(address, save_replay);
    let run_ai_client = move || self::run(socket_address, depth);
    let (handle_1, handle_2) = if rand::thread_rng().gen() {
        launch_clients(run_client, run_ai_client)
    } else {
        launch_clients(run_ai_client, run_client)
    };
    handle_1.join().unwrap();
    handle_2.join().unwrap();
}

fn launch_clients<F, G, T>(client_1: F, client_2: G) -> (JoinHandle<T>, JoinHandle<T>)
where
    T: marker::Send + 'static,
    F: marker::Send + FnOnce() -> T + 'static,
    G: marker::Send + FnOnce() -> T + 'static,
{
    let handle_1 = thread::spawn(client_1);
    thread::sleep(time::Duration::from_millis(500));
    let handle_2 = thread::spawn(client_2);
    (handle_1, handle_2)
}

// Run the client for the ai
fn run(socket_address: (String, u16), depth: usize) {
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
