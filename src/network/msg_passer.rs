use crust::PeerId;
use std::collections::HashMap;

pub struct MsgPasser {
    seq_num: u32,
    pub nodes: HashMap<PeerId, u32>,
}

impl MsgPasser {
    pub fn new() -> MsgPasser {
        MsgPasser {
            seq_num: 0,
            nodes: HashMap::new(),
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

    pub fn get_new_msg(&mut self, msg: String) -> String {
        let new_str = self.seq_num.to_string() + " " + msg.as_str();
        self.inc_seq();
        return new_str;
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

    pub fn trim_msg(msg: &String) -> Vec<&str> {
        let msgs = msg.trim_right_matches(|c| c == '\r' || c == '\n')
        .split(' ')
        .collect::<Vec<_>>();
        msgs
    }
}
