use clap::{Arg, Command};
use connect4::client;

fn main() {
    let socket_address = parse_args();
    client::run(socket_address);
}

fn cli() -> Command<'static> {
    Command::new("Connnect4 Client")
        .author("Romain Ageron & Thomas Brilland")
        .version("0.1.0")
        .about("Client for Connect4")
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
        .after_help(
            "Play either locally against an AI or online against someone else. The state of the game is maintained on the server side, not on the client side.",
        )
}

fn parse_args() -> (String, u16) {
    let app = cli();
    let matches = app.get_matches();
    let ip = matches.value_of("ipaddress").unwrap().trim().to_owned();
    let port: u16 = matches
        .value_of("port")
        .unwrap()
        .trim()
        .parse()
        .expect("Unvalid value for port. It should be an integer between 0 and 65,535.");
    (ip, port)
}
