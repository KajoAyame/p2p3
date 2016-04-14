//use std;
use std::collections::{BTreeMap, HashMap};
use crust::{Service, OurConnectionInfo, PeerId};
//use crust::{Service, Protocol, Endpoint, ConnectionInfoResult,
//            SocketAddr, OurConnectionInfo,
//            PeerId};
//use std::time::Duration;
use std::sync::{Arc, Mutex};

/*
struct Guid {
    lowBits: i64,
    higtBits: i64,
}

struct Data {
    // TODO: TBD
    uri: String,
}*/

/*
put(GUID, data)
    Stores data in replicas at all nodes responsible for the object identified by GUID.
remove(GUID)
    Deletes all references to GUID and the associated data.
value = get(GUID)
    Retrieves the data associated with GUID from one of the nodes responsible for it.
*/
/*
trait Pastry {
    fn put(id: Guid, data: Data) {
        // TODO:
    }

    fn remove(id: Guid) {
        // TODO::
    }

    fn get(id: Guid) -> Data {
        let res = Data{ uri : "some path".to_string()};
        res
        // TODO:
    }
}*/

// Make it pub for test
pub struct Network {
    pub nodes: HashMap<usize, PeerId>,
    pub our_connection_infos: BTreeMap<u32, OurConnectionInfo>,
    pub performance_start: ::time::SteadyTime,
    pub performance_interval: ::time::Duration,
    pub received_msgs: u32,
    pub received_bytes: usize,
    pub peer_index: usize,
    pub connection_info_index: u32,
}

// simple "routing table" without any structure
impl Network {
    pub fn new() -> Network {
        Network {
            nodes: HashMap::new(),
            our_connection_infos: BTreeMap::new(),
            performance_start: ::time::SteadyTime::now(),
            performance_interval: ::time::Duration::seconds(10),
            received_msgs: 0,
            received_bytes: 0,
            peer_index: 0,
            connection_info_index: 0,
        }
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

    /*
    pub fn remove_disconnected_nodes(&mut self) {
        let to_remove = self.nodes.iter().filter_map(|(id, node)| {
            if node.is_closed() {
                Some(id.clone())
            } else {
                None
            }
        }).collect::<Vec<_>>();
        for id in to_remove {
            let _ = self.nodes.remove(&id);
        }
    }
    */

    pub fn get_peer_id(&self, n: usize) -> Option<&PeerId> {
        self.nodes.get(&n)
    }

    pub fn record_received(&mut self, msg_size: usize) {
        self.received_msgs += 1;
        self.received_bytes += msg_size;
        if self.received_msgs == 1 {
            self.performance_start = ::time::SteadyTime::now();
        }
        if self.performance_start + self.performance_interval < ::time::SteadyTime::now() {
            println!("\nReceived {} messages with total size of {} bytes in last {} seconds.",
                     self.received_msgs,
                     self.received_bytes,
                     self.performance_interval.num_seconds());
            self.received_msgs = 0;
            self.received_bytes = 0;
        }
    }


}

pub fn handle_new_peer(service: &Service, protected_network: Arc<Mutex<Network>>, peer_id: PeerId) -> usize {
    let mut network = unwrap_result!(protected_network.lock());
    let peer_index = network.next_peer_index();
    let _ = network.nodes.insert(peer_index, peer_id);
    network.print_connected_nodes(service);
    peer_index
}
