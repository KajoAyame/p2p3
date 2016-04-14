use rustc_serialize::{Decodable, Decoder};
use docopt::Docopt;

static CLI_USAGE: &'static str = "
Usage:
  cli prepare-connection-info
  cli connect <our-info-id> <their-info>
  cli send <peer> <message>...
  cli send-all <message>...
  cli list
  cli clean
  cli stop
  cli help
  cli broadcast <message>...
  cli test
";

pub fn print_usage() {
    static USAGE: &'static str = r#"\
# Commands:
    prepare-connection-info                       - Prepare a connection info
    connect <our-info-id> <their-info>            - Initiate a connection to the remote peer
    send <peer> <message>                         - Send a string to the given peer
    send-all <message>                            - Send a string to all <ADJACENT> nodes
    broadcast <message>                           - Broadcast a string to <ALL> nodes
    list                                          - List existing connections and UDP sockets
    stop                                          - Exit the app
    help                                          - Print this help
# Where
    <our-file>      - The file where we'll read/write our connection info
    <their-file>    - The file where we'll read their connection info.
    <connection-id> - ID of a connection as listed using the `list` command
"#;
    println!("{}", USAGE);
}


#[derive(RustcDecodable, Debug)]
struct CliArgs {
    cmd_prepare_connection_info: bool,
    cmd_connect: bool,
    cmd_send: bool,
    cmd_send_all: bool,
    cmd_list: bool,
    cmd_broadcast: bool,
    cmd_test: bool,
    //cmd_clean: bool,
    cmd_stop: bool,
    cmd_help: bool,
    arg_peer: Option<usize>,
    arg_message: Vec<String>,
    arg_our_info_id: Option<String>,
    arg_their_info: Option<String>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UserCommand {
    Stop,
    PrepareConnectionInfo,
    Connect(String, String),
    Send(usize, String),
    SendAll(String),
    List,
    Broadcast(String),
    Test
    //Clean,
}

pub fn parse_user_command(cmd: String) -> Option<UserCommand> {
    let docopt: Docopt = Docopt::new(CLI_USAGE).unwrap_or_else(|error| error.exit());

    let mut cmds = cmd.trim_right_matches(|c| c == '\r' || c == '\n')
                      .split(' ')
                      .collect::<Vec<_>>();

    cmds.insert(0, "cli");

    let args: CliArgs = match docopt.clone().argv(cmds.into_iter()).decode() {
        Ok(args) => args,
        Err(error) => {
            match error {
                ::docopt::Error::Decode(what) => println!("{}", what),
                _ => println!("Invalid command."),
            };
            return None;
        }
    };

    if args.cmd_connect {
        let our_info_id = unwrap_option!(args.arg_our_info_id, "Missing our_info_id");
        let their_info = unwrap_option!(args.arg_their_info, "Missing their_info");
        Some(UserCommand::Connect(our_info_id, their_info))
    } else if args.cmd_send {
        let peer = unwrap_option!(args.arg_peer, "Missing peer");
        let msg = args.arg_message.join(" ");
        Some(UserCommand::Send(peer, msg))
    } else if args.cmd_send_all {
        let msg = args.arg_message.join(" ");
        Some(UserCommand::SendAll(msg))
    } else if args.cmd_prepare_connection_info {
        Some(UserCommand::PrepareConnectionInfo)
    } else if args.cmd_list {
        Some(UserCommand::List)
    } /* else if args.cmd_clean {
        Some(UserCommand::Clean)
    } */ else if args.cmd_stop {
        Some(UserCommand::Stop)
    }   // For brocast
    else if args.cmd_broadcast {
        let msg = args.arg_message.join(" ");
        Some(UserCommand::Broadcast(msg))
    } else if args.cmd_help {
        print_usage();
        None
    } else if args.cmd_test {
        Some(UserCommand::Test)
    } else {
        None
    }
}
