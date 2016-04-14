use crust::PeerId;
use std::collections::HashMap;
use ::network::message::{Message, Kind};
use bincode::rustc_serialize::{encode};
use crust::Service;
use std::sync::mpsc::channel;
pub type CrustEventSender = ::maidsafe_utilities::event_sender::MaidSafeObserver<Event>;
pub use crust::Event;

pub struct MsgPasser {
    seq_num: u32,
    pub nodes: HashMap<usize, PeerId>,
    my_id: PeerId,
    //service: Service,
}

impl MsgPasser {
    pub fn new(id: PeerId, node: HashMap<usize, PeerId>, sender: &CrustEventSender) -> MsgPasser {
            MsgPasser {
            seq_num: 0,
            nodes: node,
            my_id: id,
            //service: service,
        }
    }
    /*
    pub fn getMsg(self) -> String {
        return self.receivedMsgs;
    }*/
    pub fn get_seq_num(&self) -> u32 {
        self.seq_num
    }

    pub fn next_seq_num(&mut self) -> u32 {
        self.seq_num += 1;
        self.seq_num
    }

    pub fn inc_seq(&mut self) {
        self.seq_num += 1;
    }

    //pub fn handleBroadcast(&mut self, msg: String, id: &PeerId) -> bool {
    pub fn handle_broadcast(&mut self, msg_seq: u32) -> bool {
        if msg_seq < self.seq_num {
            return false;
        } else {
            self.inc_seq();
            return true;
        }
    }

/*
    pub fn broadcast(&mut self, message: String) {
        let mut msg = Message::new_with_kind(Kind::Broadcast, self.my_id, message);
        msg.set_seq_num(self.get_seq_num());
        let bytes = encode(&msg, ::bincode::SizeLimit::Infinite).unwrap();

        for (_, peer_id) in self.nodes.iter_mut() {
            self.service.send(peer_id, bytes.clone());
        }
        self.inc_seq();
    }
*/
    /*
    pub fn get_service(&self) -> Service {
        self.service
    }*/
}
