use crate::game_logic::Player::{self, *};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

const MAX_MESSAGE_SIZE: usize = 1_024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Hello(Player),
    Play,
    Action(u8),
    ValidAction(u8),
    InvalidAction,
    Lose,
    Draw,
    Win,
}

use Message::*;

impl Message {
    pub fn receive_from(player: &mut TcpStream) -> Self {
        let mut buffer = [0; MAX_MESSAGE_SIZE];
        player.read(&mut buffer).unwrap();
        Message::from_bytes(&buffer)
    }

    pub fn send_to(self, player: &mut TcpStream) {
        player.write(&self.to_bytes()).unwrap();
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        match bytes {
            &[0, 0, ..] => Hello(Red),
            &[0, 1, ..] => Hello(Yellow),
            &[1, 0, ..] => Play,
            &[1, 1, action, ..] => Action(action),
            &[1, 2, action, ..] => ValidAction(action),
            &[1, 3, ..] => InvalidAction,
            &[2, 0, ..] => Lose,
            &[2, 1, ..] => Draw,
            &[2, 2, ..] => Win,
            _ => panic!("bytes cannot be converted to a message: {:?}", bytes),
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        match self {
            Hello(Red) => vec![0, 0],
            Hello(Yellow) => vec![0, 1],
            Play => vec![1, 0],
            Action(action) => vec![1, 1, action],
            ValidAction(action) => vec![1, 2, action],
            InvalidAction => vec![1, 3],
            Lose => vec![2, 0],
            Draw => vec![2, 1],
            Win => vec![2, 2],
        }
    }
}
