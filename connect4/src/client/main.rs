use clap::{Arg, ArgMatches, Command};
use connect4::{ai, client, server};
use rand::Rng;

fn main() {
    let (socket_address, save_replay, play_ai) = parse_args();
    if let Some(depth) = play_ai {
        server::run(socket_address.clone());
        if rand::thread_rng().gen() {
            client::run(socket_address.clone(), save_replay);
            ai::run_client(socket_address, depth);
        } else {
            ai::run_client(socket_address.clone(), depth);
            client::run(socket_address, save_replay);
        }
    } else {
        client::run(socket_address, save_replay);
    }
}

fn cli() -> Command<'static> {
    Command::new("Connnect4 Client")
        .author("Romain Ageron & Thomas Brilland")
        .version("0.1.0")
        .about("Client for Connect4")
        .arg(
            Arg::new("ai")
                .short('a')
                .long("ai")
                .takes_value(false)
                .help("Play against an artificial intelligence on a local server"),
        )
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .default_value("4")
                .help(
                    "Depth of the search for the ai. If this argument is specified, adding the \
                    flag ai will not have any effect.",
                ),
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

fn optional_arg(arg_matches: &ArgMatches, arg: &str, flag: &str) -> Option<String> {
    if arg_matches.is_present(flag) | (arg_matches.occurrences_of(arg) > 0) {
        Some(arg_matches.value_of(arg).unwrap().trim().to_owned())
    } else {
        None
    }
}

fn parse_args() -> ((String, u16), Option<String>, Option<usize>) {
    let app = cli();
    let arg_matches = app.get_matches();
    let ip = arg_matches.value_of("ipaddress").unwrap().trim().to_owned();
    let port: u16 = arg_matches
        .value_of("port")
        .unwrap()
        .trim()
        .parse()
        .expect("Unvalid value for port. It should be an integer between 0 and 65,535.");
    let socket_address = (ip, port);
    let save_replay = optional_arg(&arg_matches, "replayfile", "savereplay")
        .map(|filename| format!("games/{}", filename));
    let play_ai = optional_arg(&arg_matches, "depth", "ai").map(|depth| {
        depth
            .parse()
            .expect("Unvalid value for depth. It should be an positive integer.")
    });
    (socket_address, save_replay, play_ai)
}
