use crust::PeerId;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Message {
    //receivedMsgs: String,
    //peerId: PeerId,
    seq_num: u32,
    source: PeerId,
    message: String,
    kind: Kind,
}

impl Message {
    pub fn new(src: PeerId, msg: String) -> Message {
        Message {
            source: src,
            seq_num: 0,
            message: msg,
            kind: Kind::Nomal,
        }
    }

    pub fn new_with_kind(k: Kind, src: PeerId, msg: String) -> Message {
        Message {
            source: src,
            seq_num: 0,
            message: msg,
            kind: k,
        }
    }
    /*
    pub fn getMsg(self) -> String {
        return self.receivedMsgs;
    }*/
    pub fn get_seq_num(&self) -> u32 {
        self.seq_num
    }

    pub fn get_msg(&self) -> String {
        self.message.clone()
    }

    pub fn get_src(&self) -> PeerId {
        self.source
    }

    pub fn get_kind(&self) -> Kind {
        self.kind.clone()
    }

    pub fn set_seq_num(&mut self, num: u32) {
        self.seq_num = num;
    }
}

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub enum Kind {
    Nomal,
    Broadcast,
}
