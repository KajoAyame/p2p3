extern crate crust;
extern crate config_file_handler;

use ::crust::{ read_config_file};
use std::env;

pub struct Network {
    name: String,
}

// simple "routing table" without any structure
impl Network {
    pub fn new() -> Network {

        let file_name = get_file_name().unwrap().into_string().unwrap();
        println!("file_name = {}", file_name);
        println!("current exe in network : {}", env::current_exe().unwrap().to_str().unwrap());

        let config = read_config_file().unwrap();
        let contacts = config.hard_coded_contacts.len();
        println!("len = {}", contacts);

        Network {
            name: file_name,
        }
    }

}
pub fn get_file_name() -> Result<::std::ffi::OsString, crust::Error> {
    let mut name = try!(config_file_handler::exe_file_stem());
    name.push(".crust.config");
    Ok(name)
}
