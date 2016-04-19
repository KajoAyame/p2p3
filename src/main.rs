extern crate curl;
extern crate crust;
#[macro_use]
extern crate maidsafe_utilities; // macro unwrap!()
extern crate p2p3;
extern crate rustc_serialize;
extern crate docopt;
extern crate service_discovery;
extern crate bincode;

use std::sync::mpsc::channel;
use std::io::prelude::*;
use rustc_serialize::json;
use std::io;
use crust::{Service, ConnectionInfoResult, StaticContactInfo};
use std::sync::{Arc, Mutex};
use p2p3::network::network_manager::Network;
use p2p3::network::network_manager::handle_new_peer;
use p2p3::network::cmd_parser;
use p2p3::network::cmd_parser::UserCommand;
use p2p3::network::cmd_parser::parse_user_command;
use p2p3::network::message::{Message, Kind};
use p2p3::network::msg_passer::MsgPasser;
use p2p3::network::bootstrap_handler::BootstrapHandler;
use p2p3::storage::storage_helper::GitAccess;
use std::thread;
use std::str::FromStr;
use bincode::rustc_serialize::{encode, decode}; // Use for encode and decode
use rustc_serialize::json::Json;


pub fn main() {
    /*
     *  Bootstrap
     */

    let git = GitAccess::new("https://github.com/KajoAyame/p2p3_test.git", "temp", "zhou.xinghao.1991@gmail.com", "123456abc");

    match git.clone() {
        Ok(()) => (),
        Err(e) => {
            println!("clone error: {}", e);
        }
    }
    let mut boot = BootstrapHandler::bootstrap_load("temp/p2p3.crust.config");

     // Construct Service and start listening
     let (channel_sender, channel_receiver) = channel();
     let (category_tx, category_rx) = channel();

 //    let (bs_sender, bs_receiver) = channel();
     let crust_event_category =
         ::maidsafe_utilities::event_sender::MaidSafeEventCategory::Crust;
     let event_sender =
         ::maidsafe_utilities::event_sender::MaidSafeObserver::new(channel_sender,
                                                                   crust_event_category,
                                                                   category_tx);

     /* If this file name is "file_name.rs",
      * it will read the config file named "file_name.crust.config".
      */
    let config = unwrap_result!(::crust::read_config_file()); // ".crust.config"

    let mut service = unwrap_result!(Service::with_config(event_sender, &config));
    unwrap_result!(service.start_listening_tcp());
    unwrap_result!(service.start_listening_utp());
    let my_id = service.id();

    /*
     *  Update the config file
     */
    // boot.update_config(git, s);

    /*
    service.prepare_connection_info(0);

    match unwrap_result!(channel_receiver.recv()) {
        crust::Event::ConnectionInfoPrepared(result) => {
            //println!("prepared!");

            let ConnectionInfoResult {
                result_token, result } = result;
            let info = result.unwrap();
            let their_info = info.to_their_connection_info();
            let info_json = unwrap_result!(json::encode(&their_info));
            //println!("Share this info with the peer you want to connect to:");
            //println!("{}", info_json);

            //let json_obj: Json = input_data.to_json();
            let data = Json::from_str(info_json.as_str()).unwrap();
            let obj = data.as_object().unwrap();
            let foo = obj.get("static_contact_info").unwrap();
            //println!("foo = \n{}", foo);

            let json_str: String = foo.to_string();
            //println!("json_str = \n{}", json_str);

            let mut info: StaticContactInfo = json::decode(&json_str).unwrap();
            info.tcp_acceptors.remove(0);

            boot.update_config(git, info);
        }
        event => panic!("Received unexpected event: {:?}", event),
    }*/

    //println!("/////// END ///////");



     // Enable listening and responding to peers searching for us.
     service.start_service_discovery();

     let service = Arc::new(Mutex::new(service));
     let service_cloned = service.clone();

     let network = Arc::new(Mutex::new(Network::new()));
     let network2 = network.clone();

     let msg_handler = Arc::new(Mutex::new(MsgPasser::new()));
     let msg_handler2 = msg_handler.clone();

     /* Start event-handling thread.
      * This thread handles the peer events.
      */
     let handler = match thread::Builder::new().name("CrustNode event handler".to_string())
                                               .spawn(move || {
         let service = service_cloned;
         for it in category_rx.iter() {
             match it {
                 ::maidsafe_utilities::event_sender::MaidSafeEventCategory::Crust => {
                     if let Ok(event) = channel_receiver.try_recv() {
                         match event {
                             // Invoked when a new message is received. Passes the message.
                             crust::Event::NewMessage(peer_id, bytes) => {
                                 //let message_length = bytes.len();
                                 let mut network = unwrap_result!(network2.lock());
                                 // network.record_received(message_length);

                                 let decoded_msg: Message = decode(&bytes[..]).unwrap();
                                 let msg = decoded_msg.get_msg();

                                 /*
                                  * Handle brocast message
                                  */
                                 match decoded_msg.get_kind() {
                                     Kind::Broadcast => {
                                         if decoded_msg.get_src() != my_id &&
                                             unwrap_result!(msg_handler2.lock()).handle_broadcast(decoded_msg.get_seq_num()) {
                                             println!("\nReceived from {:?} message: {:?}",
                                                      peer_id, msg);

                                             for (_, peer_id) in network.nodes.iter_mut() {
                                                 unwrap_result!(unwrap_result!(service.lock()).send(peer_id, bytes.clone()));
                                             }


                                         }
                                     }

                                     Kind::Nomal => {
                                         println!("\nReceived from {:?} message: {:?}",
                                                  peer_id, msg);
                                     }
                                 }
                             },
                             // Invoked as a result to the call of Service::prepare_contact_info.
                             crust::Event::ConnectionInfoPrepared(result) => {
                                 let ConnectionInfoResult {
                                     result_token, result } = result;
                                 let info = match result {
                                     Ok(i) => i,
                                     Err(e) => {
                                         println!("Failed to prepare connection info\ncause: {}", e);
                                         continue;
                                     }
                                 };
                                 println!("Prepared connection info with id {}", result_token);
                                 let their_info = info.to_their_connection_info();
                                 let info_json = unwrap_result!(json::encode(&their_info));
                                //
                                let data = Json::from_str(info_json.as_str()).unwrap();
                                let obj = data.as_object().unwrap();
                                let foo = obj.get("static_contact_info").unwrap();
                                //println!("foo = \n{}", foo);

                                let json_str: String = foo.to_string();
                                //println!("json_str = \n{}", json_str);

                                let mut info: StaticContactInfo = json::decode(&json_str).unwrap();
                                info.tcp_acceptors.remove(0);

                                boot.update_config(&git, info);
                                 //
                                 /*
                                 println!("Share this info with the peer you want to connect to:");
                                 println!("{}", info_json);
                                 let mut network = unwrap_result!(network2.lock());
                                 if let Some(_) = network.our_connection_infos.insert(result_token, info) {
                                     panic!("Got the same result_token twice!");
                                 };*/
                             },
                             // Invoked when we get a bootstrap connection to a new peer.
                             crust::Event::BootstrapConnect(peer_id) => {
                                 println!("\nBootstrapConnect with peer {:?}", peer_id);
                                 handle_new_peer(&unwrap_result!(service.lock()), network2.clone(), peer_id);
                                 //let peer_index = handle_new_peer(&unwrap_result!(service.lock()), network2.clone(), peer_id);
                                 //let _ = bs_sender.send(peer_index);
                             },
                             // Invoked when a bootstrap peer connects to us.
                             crust::Event::BootstrapAccept(peer_id) => {
                                 println!("\nBootstrapAccept with peer {:?}", peer_id);
                                 handle_new_peer(&unwrap_result!(service.lock()), network2.clone(), peer_id);
                                 //let peer_index = handle_new_peer(&unwrap_result!(service.lock()), network2.clone(), peer_id);
                                 //let _ = bs_sender.send(peer_index);
                             },
                             // Invoked when a connection to a new peer is established.
                             crust::Event::NewPeer(Ok(()), peer_id) => {
                                 println!("\nConnected to peer {:?}", peer_id);
                                 let _ = handle_new_peer(&unwrap_result!(service.lock()), network2.clone(), peer_id);
                             }
                             // Invoked when a peer is lost.
                             crust::Event::LostPeer(peer_id) => {
                                 println!("\nLost connection to peer {:?}",
                                          peer_id);
                                 let mut index = None;
                                 {
                                     let network = unwrap_result!(network2.lock());
                                     for (i, id) in network.nodes.iter() {
                                         if id == &peer_id {
                                             index = Some(*i);
                                             break;
                                         }
                                     }
                                 }
                                 let mut network = unwrap_result!(network2.lock());
                                 if let Some(index) = index {
                                     let _ = unwrap_option!(network.nodes.remove(&index), "index should definitely be a key in this map");
                                 };
                                 network.print_connected_nodes(&unwrap_result!(service.lock()));
                             }
                             e => {
                                 println!("\nReceived event {:?} (not handled)", e);
                             }
                         }

                     } else {
                         break;
                     }
                 },
                 _ => unreachable!("This category should not have been fired - {:?}", it),
             }
         }
     }) {
         Ok(join_handle) => join_handle,
         Err(e) => {
             println!("Failed to start event-handling thread: {}", e);
             std::process::exit(5);
         },
     };

     unwrap_result!(service.lock()).prepare_connection_info(0);
     cmd_parser::print_usage();
 //    println!("Debug string: 123");

     /*
      * Main thread handles the user input command line.
      */
     loop {
         use std::io::Write; // For flush().

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
                 let mut network = unwrap_result!(network.lock());
                 let token = network.next_connection_info_index();
                 println!("token = {}", token);
                 unwrap_result!(service.lock()).prepare_connection_info(token);
             }
             UserCommand::Connect(our_info_index, their_info) => {
                 let mut network = unwrap_result!(network.lock());
                 let our_info_index = match u32::from_str(&our_info_index) {
                     Ok(info) => info,
                     Err(e) => {
                         println!("Invalid connection info index: {}", e);
                         continue;
                     },
                 };
                 let our_info = match network.our_connection_infos.remove(&our_info_index) {
                     Some(info) => info,
                     None => {
                         println!("Invalid connection info index");
                         continue;
                     },
                 };
                 let their_info = match json::decode(&their_info) {
                     Ok(info) => info,
                     Err(e) => {
                         println!("Error decoding their connection info");
                         println!("{}", e);
                         continue;
                     },
                 };
                 unwrap_result!(service.lock()).connect(our_info, their_info);
             }
             UserCommand::Send(peer_index, message) => {
                 let network = unwrap_result!(network.lock());

                 let msg = Message::new(my_id, message);
                 let bytes = encode(&msg, bincode::SizeLimit::Infinite).unwrap();

                 match network.get_peer_id(peer_index) {
                     Some(ref mut peer_id) => {
                         unwrap_result!(unwrap_result!(service.lock()).send(peer_id, bytes));
                     }
                     None => println!("Invalid connection #"),
                 }
             }
             UserCommand::SendAll(message) => {
                 let mut network = unwrap_result!(network.lock());
                 let msg = Message::new(my_id, message);
                 let bytes = encode(&msg, bincode::SizeLimit::Infinite).unwrap();
                 for (_, peer_id) in network.nodes.iter_mut() {
                     unwrap_result!(unwrap_result!(service.lock()).send(peer_id, bytes.clone()));
                 }
             }
             UserCommand::List => {
                 let network = unwrap_result!(network.lock());
                 network.print_connected_nodes(&unwrap_result!(service.lock()));
             }
             UserCommand::Broadcast(message) => {
                 let mut network = unwrap_result!(network.lock());
                 let mut msg = Message::new_with_kind(Kind::Broadcast, my_id, message);
                 msg.set_seq_num(unwrap_result!(msg_handler.lock()).get_seq_num());
                 let bytes = encode(&msg, bincode::SizeLimit::Infinite).unwrap();

                 for (_, peer_id) in network.nodes.iter_mut() {
                     unwrap_result!(unwrap_result!(service.lock()).send(peer_id, bytes.clone()));
                 }
                 unwrap_result!(msg_handler.lock()).inc_seq();
             }
             UserCommand::Test => {
                 println!("my id is: {}", unwrap_result!(service.lock()).id());
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

     drop(service);
     assert!(handler.join().is_ok());

}
