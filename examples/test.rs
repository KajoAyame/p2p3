extern crate crust;
extern crate config_file_handler;
extern crate p2p3;

use crust::{Service, ConnectionInfoResult, read_config_file, write_config_file};
use std::env;
use p2p3::network::network_manager::Network;


fn main() {
    //let mut name = try!(config_file_handler::exe_file_stem());
    //println!("file name: {}", get_file_name().unwrap().into_string().unwrap());
    //let cfg = try!(file_handler.read_file());
    //println!("current exe: {}", env::current_exe().unwrap().to_str().unwrap());

    println!("");
    let network = Network::new();
}


/*
pub fn read_config_file() -> Result<Config, crust::Error> {
    let file_handler = try!(FileHandler::new(&try!(get_file_name())));
    let cfg = try!(file_handler.read_file());
    Ok(cfg)
}*/
