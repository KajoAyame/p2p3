extern crate config_file_handler;

use curl::http;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

/*
 *  bootstrap_download: Download the config file and store it in the path that
 *  Crust uses to read.
 */
pub fn bootstrap_download(url: &str) {
    let resp = http::handle()
    //.get("http://54.209.245.74:8080/--bootstrap")
    //.get("https://github.com/KajoAyame/p2p3/blob/master/target/debug/p2p3.crust.config")
    .get(url)
    .exec().unwrap();


    // Get the config file content
    let config_u8 = resp.get_body();

    // Get the config file path
    let file_name = get_file_name().unwrap().into_string().unwrap();
    println!("file_name = {}", file_name);
    let path_str = "target/debug/".to_string() + &file_name; // "target/debug/" in stead of "/target/debug/"
    println!("path = {}", path_str);

    // Store it in the path
    let path = Path::new(&path_str);
    let mut f = File::create(path.clone()).unwrap();
    f.write_all(config_u8).unwrap();

    // Read it
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    println!("{}", s);

    // Read it into Config
    let config = unwrap_result!(::crust::read_config_file());
    let contacts = config.hard_coded_contacts.len();
    println!("len = {}", contacts);
}

pub fn get_file_name() -> Result<::std::ffi::OsString, ::crust::Error> {
    let mut name = try!(config_file_handler::exe_file_stem());
    name.push(".crust.config");
    Ok(name)
}

pub fn update_config() {

}
