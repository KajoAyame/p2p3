#![allow(dead_code,unused_variables,unused_imports)]
use super::woot_char::WootChar;

#[derive(Clone,PartialEq,Debug,RustcDecodable,RustcEncodable)]
pub enum Operation {
    Insert {w_char: WootChar, from_site: u32},
    Delete {w_char: WootChar, from_site: u32}
}
unsafe impl Send for Operation {}
unsafe impl Sync for Operation {}
