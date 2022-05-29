use clap::{Arg, Command};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn cli() -> Command<'static> {
    Command::new("Connnect4 Server")
        .author("Romain Ageron & Thomas Brilland")
        .version("0.1.0")
        .about("Online server for Connect4")
        .arg(
            Arg::new("encoding")
                .short('c')
                .long("codec")
                .default_value("ber")
                .help("Message encoding: ber, der, gser, jer, oer, per, uper or xer"),
        )
        .arg(
            Arg::new("ipaddress")
                .short('i')
                .long("ip")
                .default_value("127.0.0.1")
                .help("IP address of the server"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .default_value("50001")
                .help("Port on which the server is listening"),
        )
        .arg(
            Arg::new("specification")
                .short('s')
                .long("spec")
                .default_value("./TicTacToe.asn")
                .help("Protocole specification file in ASN.1"),
        )
        .after_help(
            "The main thread handles connections. As soon as two players are in the queue, \
            the game starts on a dedicated thread. The state of the game is maintained on the \
            server side, not on the client side.",
        )
}

fn parse_args() -> ((String, u16), String, String) {
    let app = cli();
    let args = app.get_matches();
    let ip = args.value_of("ipaddress").unwrap().trim().to_owned();
    let port: u16 = args
        .value_of("port")
        .unwrap()
        .trim()
        .parse()
        .expect("Unvalid value for port. It should be an integer between 0 and 65,535.");
    let codec = args.value_of("encoding").unwrap().trim().to_owned();
    let spec = args.value_of("specification").unwrap().trim().to_owned();
    let socket_address = (ip, port);
    (socket_address, codec, spec)
}

fn dialog(mut player_1: TcpStream, mut player_2: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        player_1.read(&mut buffer).unwrap();
        player_2.write(&buffer).unwrap();
        player_2.read(&mut buffer).unwrap();
        player_1.write(&buffer).unwrap();
    }
}

fn main() {
    let (socket_address, codec, spec) = parse_args();
    let listener = TcpListener::bind(socket_address).unwrap();
    let mut queue = Vec::with_capacity(2);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("{:#?}", stream);
                queue.push(stream);
            }
            Err(e) => continue,
        }
        if queue.len() == 2 {
            dialog(queue.pop().unwrap(), queue.pop().unwrap());
        }
    }
}
