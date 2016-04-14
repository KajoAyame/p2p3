extern crate curl;
extern crate crust;
#[macro_use]
extern crate maidsafe_utilities; // macro unwrap!()
extern crate p2p3;

use curl::http;
use std::str;
use std::io::prelude::*;
use std::fs::File;
use p2p3::network::network::get_file_name;
use std::path::Path;

pub fn main() {
  let resp = http::handle()
    //.get("http://54.209.245.74:8080/--bootstrap")
    //.get("https://github.com/KajoAyame/p2p3/blob/master/target/debug/p2p3.crust.config")
    .get("https://raw.githubusercontent.com/KajoAyame/p2p3/master/target/debug/p2p3.crust.config")
    .exec().unwrap();


    //println!("code={}; headers={:?}; body={:?}",
    //resp.get_code(), resp.get_headers(), resp.get_body());
    let config_u8 = resp.get_body();
    let config_str = str::from_utf8(&config_u8).unwrap();

    //println!("body = {}", config_str);

    let file_name = get_file_name().unwrap().into_string().unwrap();
    println!("file_name = {}", file_name);

    //let mut path_str = file_name + "/target/debug/";
    let mut path_str = "target/debug/".to_string() + &file_name; // "target/debug/" in stead of "/target/debug/"
    println!("path = {}", path_str);

    let path = Path::new(&path_str);
    let mut f = File::create(path.clone()).unwrap();
    f.write_all(config_u8).unwrap();

    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    println!("{}", s);

    let config = unwrap_result!(::crust::read_config_file());
    let contacts = config.hard_coded_contacts.len();
    println!("len = {}", contacts);
}

// Post
/*
fn main(){
  let resp = http::handle()
    .post("http://localhost:3000/login", "username=dude&password=sikrit")
    .exec().unwrap();

  println!("code={}; headers={}; body={}",
    resp.get_code(), resp.get_headers(), resp.get_body());

}*/
