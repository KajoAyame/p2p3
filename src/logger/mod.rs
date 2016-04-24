use network::{Message,MsgKind};
use async_queue::AsyncQueue;
use std::sync::Arc;
use std::collections::VecDeque;

pub struct Logger {
    messages: Arc<AsyncQueue<Message>>
}

impl Logger {
    pub fn init() -> Logger {
        Logger {
            messages: Arc::new(AsyncQueue::new())
        }
    }

    pub fn reception(&self, message: Message) {
        let logger_queue = self.messages.clone();
        logger_queue.enq(message);
    }

    pub fn print(&self) {
        let logger_queue = self.messages.clone();
        let q = logger_queue.queue.lock().unwrap();
        for c in q.iter() {
            println!("{}: From: {} Content: {} Kind: {:?}", c.seq_num, c.source, c.message, c.kind);
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    extern crate crust;
    use crust::PeerId;
    use sodiumoxide::crypto::box_::PublicKey;
    #[test]
    fn test_logger() {
        let logger = Logger::init();
        let mut rng = rand::thread_rng();
        let random_num = Range::new(0, 100);
        let peer_id = PeerId::new();
        let message1 = Message {
            source: peer_id.rand(&mut rng),
            message: "Woot Operation to insert 'a'".to_string(),
            kind: MsgKind::Broadcast,
            seq_num: 0};
        logger.reception(message1);

        let message2 = Message {
            source: peer_id.rand(&mut rng),
            message: "Woot Operation to insert 'b'".to_string(),
            kind: MsgKind::Broadcast,
            seq_num: 0};
        logger.reception(message1);
        logger.reception(message2);
        {
            let logger_queue = self.messages.clone();
            let q = logger_queue.queue.lock().unwrap();
            assert_eq!(q.len(), 2);
        }
        logger.print();
    }
}
