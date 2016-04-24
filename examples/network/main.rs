extern crate p2p3;
extern crate crust;
extern crate docopt;
extern crate rustc_serialize;
#[macro_use]
extern crate maidsafe_utilities;

mod cmd_parser;
use cmd_parser::*;

use std::io::Write;
use std::io;
use rustc_serialize::json;
use p2p3::network::MessagePasserT;
use p2p3::network::bootstrap::BootstrapHandler;
use p2p3::storage::storage_helper::GitAccess;
use std::str::FromStr;

fn main() {

    // Get the four parameters from the front-end.
    let repo_url: String = "https://github.com/KajoAyame/p2p3_test.git".to_string();
    let local_url: String = "temp/".to_string();
    let file_path: String = "file1.rs".to_string();
    let username: String = "zhou.xinghao.1991@gmail.com".to_string();
    let password: String = "123456abc".to_string();


    let git = GitAccess::new(repo_url, local_url, file_path, username, password);
    match git.clone_repo() {
        Ok(()) => (),
        Err(e) => {
            println!("{}, stop clone and read the config file in that directory", e);
        }
    }

    println!("Starting bootstrap");

    // Get file name from the front end.
    let boot = BootstrapHandler::bootstrap_load(git);

    // Network
    print_usage();
    let (mp,_) = p2p3::network::MessagePasser::new();
    boot.update_config(mp.clone());
    loop {
        print!("> ");
        assert!(io::stdout().flush().is_ok());

        // Get the command line from user input
        let mut command = String::new();
        assert!(io::stdin().read_line(&mut command).is_ok());

        let cmd = match parse_user_command(command) {
            Some(cmd) => cmd,
            None => continue,
        };

        match cmd {
            UserCommand::PrepareConnectionInfo => {
                let tok = mp.prepare_connection_info();
                let con = mp.wait_conn_info(tok);
                println!("Share this with other client:\n{}",json::encode(&con).unwrap());
            }
            UserCommand::Connect(our_info_index, their_info) => {
                let their_info = unwrap_result!(json::decode(&their_info));
                let index = u32::from_str(our_info_index.as_str()).unwrap();
                mp.connect(index, their_info);
            }
            UserCommand::Send(index, message) => {
                let peers = mp.peers();
                //let index = usize::from_str(peer_index).unwrap();
                mp.send(peers[index], message).unwrap();
            }
            UserCommand::SendAll(message) => {
                mp.broadcast(message).unwrap();
            }
            UserCommand::List => {
                let peers = mp.peers();
                let service_am = mp.get_service();
                let service = unwrap_result!(service_am.lock());
                let mut i = 0;
                for peer in peers.iter(){
                    if let Some(conn_info) = service.connection_info(peer) {
                        println!("    [{}] {}   {} <--> {} [{}][{}]",
                                 i, peer, conn_info.our_addr, conn_info.their_addr, conn_info.protocol,
                                 if conn_info.closed { "closed" } else { "open" }
                        );
                    }
                    i+=1;
                }
            }
            UserCommand::Broadcast(message) => {
                mp.broadcast(message).unwrap();
            }
            UserCommand::Test => {
                println!("Hello");
            }
            /*
            UserCommand::Clean => {
                let mut network = network.lock().unwrap();
                network.remove_disconnected_nodes();
                network.print_connected_nodes();
            }
            */
            UserCommand::Stop => {
                break;
            }
        }
    }
}
