extern crate config_file_handler;

//use curl::http;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use ::storage::storage_helper::GitAccess;
use crust::StaticContactInfo;
use socket_addr::SocketAddr;
use rustc_serialize::json;


#[derive(PartialEq, Eq, Debug, RustcDecodable, RustcEncodable, Clone)]
pub struct Config {
    pub hard_coded_contacts: Vec<StaticContactInfo>,
    pub enable_tcp: bool,
    pub enable_utp: bool,
    pub tcp_acceptor_port: Option<u16>,
    pub utp_acceptor_port: Option<u16>,
    pub udp_mapper_servers: Vec<SocketAddr>,
    pub tcp_mapper_servers: Vec<SocketAddr>,
    pub service_discovery_port: Option<u16>,
    pub bootstrap_cache_name: Option<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            hard_coded_contacts: vec![], // No hardcoded endpoints
            enable_tcp: true,
            enable_utp: true,
            tcp_acceptor_port: None,
            utp_acceptor_port: None,
            udp_mapper_servers: vec![],
            tcp_mapper_servers: vec![],
            service_discovery_port: None,
            bootstrap_cache_name: None,
        }
    }
}

pub struct BootstrapHandler {
    pub config: Config,
}

impl BootstrapHandler {
    pub fn bootstrap_load(url: &str) -> BootstrapHandler{
        let mut file = File::open(url).unwrap();
        let mut file_str = String::new();
        file.read_to_string(&mut file_str).unwrap();
        //println!("file: \n{}", file_str);
        // Get the config file path
        let file_name = get_file_name().unwrap().into_string().unwrap();
        //println!("file_name = {}", file_name);
        let path_str = "target/debug/".to_string() + &file_name; // "target/debug/" in stead of "/target/debug/"
        //println!("path = {}", path_str);

        // Store it in the path
        let path = Path::new(&path_str);
        let mut f = File::create(path.clone()).unwrap();

        let file_byte = file_str.into_bytes();
        f.write_all(&file_byte).unwrap();
        /*
        for byte in file_byte {
            f.write_all(&file_byte).unwrap();
        }*/

        // Read it
        let mut f = File::open(path).unwrap();
        let mut config_str = String::new();
        f.read_to_string(&mut config_str).unwrap();
        //println!("{}", config_str);

        // Read it into Config
        //let con = unwrap_result!(::crust::read_config_file());

        let conf: Config = json::decode(&config_str).unwrap();

        BootstrapHandler {
            config: conf,
        }
    }

    /*
    pub fn bootstrap_download(url: &str) -> BootstrapHandler{
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
        let mut config_str = String::new();
        f.read_to_string(&mut config_str).unwrap();
        println!("{}", config_str);

        // Read it into Config
        let con = unwrap_result!(::crust::read_config_file());
        let contacts = con.hard_coded_contacts.len();
        println!("len = {}", contacts);

        let conf: Config = json::decode(&config_str).unwrap();

        BootstrapHandler {
            config: conf,
            file_path: path_str.as_str()
        }
    }*/

    pub fn update_config(&mut self, git: &GitAccess, info: StaticContactInfo) {
        self.config.hard_coded_contacts[0].tcp_acceptors.insert(0, info.tcp_acceptors[0]);
        let update_str = json::encode(&self.config).unwrap();

        // Get the config file path
        let file_name = get_file_name().unwrap().into_string().unwrap();
        //println!("file_name = {}", file_name);
        let path_str = "temp/".to_string() + &file_name; // "target/debug/" in stead of "/target/debug/"
        //println!("path = {}", path_str);

        // Store it in the path
        let path = Path::new(&path_str);
        let mut file = File::create(path.clone()).unwrap();

        let file_byte = update_str.into_bytes();
        file.write_all(&file_byte).unwrap();

        //println!("******* commit *******");
        match git.commit_path("Update config file.", "p2p3.crust.config") {
            Ok(()) => (),
            Err(e) => {
                println!("Commit error: {}", e);
            }
        }

        match git.push() {
            Ok(()) => (),
            Err(e) => {
                println!("Push error: {}", e);
            }
        }
    }
}
/*
 *  bootstrap_download: Download the config file and store it in the path that
 *  Crust uses to read.
 */


pub fn get_file_name() -> Result<::std::ffi::OsString, ::crust::Error> {
    let mut name = try!(config_file_handler::exe_file_stem());
    name.push(".crust.config");
    Ok(name)
}
