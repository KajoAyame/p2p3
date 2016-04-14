#![allow(warnings)]
//use std;

use std::collections::{BTreeMap, HashMap};
use crust::{Service, OurConnectionInfo, PeerId, StaticContactInfo, Event};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver};
use ::network::message::{Message, Kind};
use maidsafe_utilities::event_sender::MaidSafeEventCategory;
use bincode::rustc_serialize::{encode, decode}; // Use for encode and decode

// Make it pub for test
pub struct Network {
    pub nodes: HashMap<usize, PeerId>,
    pub our_connection_infos: BTreeMap<u32, OurConnectionInfo>,
    pub peer_index: usize,
    pub connection_info_index: u32,
    pub config: Vec<StaticContactInfo>,
    //
    seq_num: u32,
    my_id: PeerId,
    pub service: Service,
    pub category_rx: Receiver<MaidSafeEventCategory>,
    pub channel_receiver: Receiver<Event>,
}

// simple "routing table" without any structure
impl Network {
    pub fn new() -> Network {
        let (channel_sender, receiver) = channel();
        let (category_tx, rx) = channel();

        let crust_event_category =
            ::maidsafe_utilities::event_sender::MaidSafeEventCategory::Crust;
        let event_sender =
            ::maidsafe_utilities::event_sender::MaidSafeObserver::new(channel_sender,
                                                                      crust_event_category,
                                                                      category_tx);
        /* If this file name is "file_name.rs",
         * it will read the config file named "file_name.crust.config".
         */
        //let config = unwrap_result!(::crust::read_config_file()); // ".crust.config"

        //let mut service = unwrap_result!(Service::with_config(event_sender, &config));
        let mut service = unwrap_result!(Service::new(event_sender));

        Network {
            nodes: HashMap::new(),
            our_connection_infos: BTreeMap::new(),
            peer_index: 0,
            connection_info_index: 0,
            config: vec![],
            //
            seq_num: 0,
            my_id: service.id(),
            service: service,
            category_rx: rx,
            channel_receiver: receiver,
        }
    }
    pub fn get_rx(&self) -> Receiver<MaidSafeEventCategory> {
        self.category_rx
    }
    pub fn get_nodes(&self) -> HashMap<usize, PeerId> {
        self.nodes.clone()
    }

    pub fn next_peer_index(&mut self) -> usize {
        let ret = self.peer_index;
        self.peer_index += 1;
        ret
    }

    pub fn next_connection_info_index(&mut self) -> u32 {
        let ret = self.connection_info_index;
        self.connection_info_index += 1;
        ret
    }

    pub fn print_connected_nodes(&self, service: &Service) {
        println!("Node count: {}", self.nodes.len());
        for (id, node) in self.nodes.iter() {
            /*
             * TODO(canndrew): put this back
            let status = if !node.is_closed() {
                "Connected   "
            } else {
                "Disconnected"
            };
            */

            if let Some(conn_info) = service.connection_info(node) {
                println!("    [{}] {}   {} <--> {} [{}][{}]",
                         id, node, conn_info.our_addr, conn_info.their_addr, conn_info.protocol,
                         if conn_info.closed { "closed" } else { "open" }
                );
            }
        }

        println!("");
    }

    pub fn get_peer_id(&self, n: usize) -> Option<&PeerId> {
        self.nodes.get(&n)
    }
    /*
    pub fn get_service(&self) -> Service {
        self.service
    }*/

    pub fn handle_broadcast(&mut self, msg_seq: u32) -> bool {
        if msg_seq < self.seq_num {
            return false;
        } else {
            self.inc_seq();
            return true;
        }
    }
    pub fn inc_seq(&mut self) {
        self.seq_num += 1;
    }

    pub fn read_config(&mut self) {
        let conf = unwrap_result!(::crust::read_config_file()).hard_coded_contacts;
        self.config = conf;
    }

    pub fn my_id(&self) -> PeerId {
        self.my_id
    }

    pub fn handle_receive(&mut self) {
        //let service = self.service;
        for it in self.category_rx.iter() {
            match it {
                ::maidsafe_utilities::event_sender::MaidSafeEventCategory::Crust => {
                    if let Ok(event) = self.channel_receiver.try_recv() {
                        match event {
                            // Invoked when a new message is received. Passes the message.
                            Event::NewMessage(peer_id, bytes) => {
                                //let message_length = bytes.len();
                                //let mut network = unwrap_result!(network2.lock());
                                // network.record_received(message_length);

                                let decoded_msg: Message = decode(&bytes[..]).unwrap();
                                let msg = decoded_msg.get_msg();

                                /*
                                 * Handle brocast message
                                 */
                                match decoded_msg.get_kind() {
                                    Kind::Broadcast => {
                                        if decoded_msg.get_src() != self.my_id &&
                                            self.handle_broadcast(decoded_msg.get_seq_num()) {
                                            println!("\nReceived from {:?} message: {:?}",
                                                     peer_id, msg);

                                            for (_, peer_id) in self.nodes.iter_mut() {
                                                self.service.send(peer_id, bytes.clone());
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
                            /*
                            Event::ConnectionInfoPrepared(result) => {
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
                                println!("Share this info with the peer you want to connect to:");
                                println!("{}", info_json);
                                let mut network = unwrap_result!(network2.lock());
                                if let Some(_) = network.our_connection_infos.insert(result_token, info) {
                                    panic!("Got the same result_token twice!");
                                };
                            },*/
                            // Invoked when we get a bootstrap connection to a new peer.
                            Event::BootstrapConnect(peer_id) => {
                                println!("\nBootstrapConnect with peer {:?}", peer_id);
                                let peer_index = self.next_peer_index();
                                let _ = self.nodes.insert(peer_index, peer_id);
                                self.print_connected_nodes(&self.service);
                                //handle_new_peer(self, peer_id);
                                //let peer_index = handle_new_peer(&unwrap_result!(service.lock()), network2.clone(), peer_id);
                                //let _ = bs_sender.send(peer_index);
                            },
                            // Invoked when a bootstrap peer connects to us.
                            Event::BootstrapAccept(peer_id) => {
                                println!("\nBootstrapAccept with peer {:?}", peer_id);
                                //handle_new_peer(peer_id);
                                //let peer_index = handle_new_peer(&unwrap_result!(service.lock()), network2.clone(), peer_id);
                                //let _ = bs_sender.send(peer_index);
                            },
                            // Invoked when a connection to a new peer is established.
                            Event::NewPeer(Ok(()), peer_id) => {
                                println!("\nConnected to peer {:?}", peer_id);
                                //let _ = handle_new_peer(peer_id);
                            }
                            // Invoked when a peer is lost.
                            /*
                            Event::LostPeer(peer_id) => {
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
                            }*/
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
    }
}
pub fn handle_new_peer(network: &Network, peer_id: PeerId) -> usize {
    let peer_index = network.next_peer_index();
    let _ = network.nodes.insert(peer_index, peer_id);
    network.print_connected_nodes(&network.service);
    peer_index
}
/*
pub fn handle_new_peer(service: &Service, protected_network: Arc<Mutex<Network>>, peer_id: PeerId) -> usize {
    let mut network = unwrap_result!(protected_network.lock());
    let peer_index = network.next_peer_index();
    let _ = network.nodes.insert(peer_index, peer_id);
    network.print_connected_nodes(service);
    peer_index
}*/
