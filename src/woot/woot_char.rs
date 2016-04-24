#![allow(dead_code,unused_variables,unused_imports)]

use super::char_id::CharId;

#[derive(Clone,PartialEq,Debug,RustcDecodable,RustcEncodable)]
pub struct WootChar {
    pub id: CharId,
    pub visible: bool,
    pub value: char,
    pub prev_id: CharId,
    pub next_id: CharId,
}

impl WootChar {
    pub fn new(id: CharId, value: char, prev_id: CharId, next_id: CharId) -> WootChar {
        WootChar {id: id, visible: true, value: value, prev_id: prev_id, next_id: next_id}
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }
}
