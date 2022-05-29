use crate::{
    communication::Message::{self, *},
    game_logic::{Connect4, Player::*},
};
use std::{
    net::{TcpListener, TcpStream},
    thread,
};

fn play_turn(game: &mut Connect4, player_1: &mut TcpStream, player_2: &mut TcpStream) {
    let (player, other) = game.to_play.select(player_1, player_2);
    let action = match Message::receive_from(player) {
        Action(action) if game.valid_action(action as usize) => action,
        _ => {
            InvalidAction.send_to(player);
            return;
        }
    };
    ValidAction(action).send_to(player);
    ValidAction(action).send_to(other);
    game.play(action as usize);
}

fn game_over<'a>(game: &Connect4, player_1: &'a mut TcpStream, player_2: &'a mut TcpStream) {
    if let Some(winner) = game.has_winner() {
        let (winner, loser) = winner.select(player_1, player_2);
        Win.send_to(winner);
        Lose.send_to(loser);
    } else {
        Draw.send_to(player_1);
        Draw.send_to(player_2);
    }
}

fn play_game(mut player_1: TcpStream, mut player_2: TcpStream) {
    Hello(Red).send_to(&mut player_1);
    Hello(Red).send_to(&mut player_2);
    let mut game = Connect4::new();
    while !game.over() {
        play_turn(&mut game, &mut player_1, &mut player_2)
    }
    game_over(&game, &mut player_1, &mut player_2)
}

pub fn run(socket_address: (String, u16)) {
    let listener = TcpListener::bind(socket_address).unwrap();
    let mut queue = None;
    for stream in listener.incoming() {
        if let Ok(player) = stream {
            match queue {
                Some(other_player) => {
                    queue = None;
                    thread::spawn(|| {
                        play_game(other_player, player);
                    });
                }
                None => queue = Some(player),
            }
        }
    }
}
