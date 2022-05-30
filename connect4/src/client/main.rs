use clap::{Arg, Command};
use connect4::client;

fn main() {
    let (socket_address, save_replay) = parse_args();
    client::run(socket_address, save_replay);
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
        .arg(
            Arg::new("replayfile")
                .short('r')
                .long("replay")
                .default_value("game.txt")
                .help(
                    "Save the game in a replay file in the \"games\" directory. If this argument \
                    is specified, adding the flag savereplay will not have any effect.",
                ),
        )
        .arg(
            Arg::new("savereplay")
                .short('s')
                .long("save")
                .takes_value(false)
                .help(
                    "Save the game in a replay file. If only this flag is given, the game is \
                    saved in the default file.",
                ),
        )
        .after_help(
            "Play either locally against an AI or online against someone else. The state of the \
            game is maintained on the server side, not on the client side.",
        )
}

fn parse_args() -> ((String, u16), Option<String>) {
    let app = cli();
    let matches = app.get_matches();
    let ip = matches.value_of("ipaddress").unwrap().trim().to_owned();
    let port: u16 = matches
        .value_of("port")
        .unwrap()
        .trim()
        .parse()
        .expect("Unvalid value for port. It should be an integer between 0 and 65,535.");
    let socket_address = (ip, port);
    let save_replay =
        if matches.is_present("savereplay") | (matches.occurrences_of("replayfile") > 0) {
            let filename = matches.value_of("replayfile").unwrap().trim();
            Some(format!("games/{}", filename))
        } else {
            None
        };
    (socket_address, save_replay)
}
