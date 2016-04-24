extern crate config_file_handler;
extern crate crust;
extern crate socket_addr;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use ::storage::storage_helper::GitAccess;
use self::crust::{TheirConnectionInfo,StaticContactInfo};
use self::socket_addr::SocketAddr;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::as_pretty_json;
use super::MessagePasser;


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

#[derive(Clone)]
pub struct BootstrapHandler {
    pub config: Config,
    pub git: GitAccess,
    pub file_name: String
}

impl BootstrapHandler {
    pub fn bootstrap_load(git: GitAccess) -> BootstrapHandler{
        // Load the p2p3 config file in the same directory of the working file.
        let p2p3_file_name: String = get_p2p3_config(&git.file_url);
        let p2p3_file_url = git.local_url.clone() + &p2p3_file_name;
        let mut p2p3_file = File::open(p2p3_file_url).unwrap();
        let mut file_str = String::new();
        p2p3_file.read_to_string(&mut file_str).unwrap();

        // Get the crust config file path
        let file_name = get_crust_config().unwrap().into_string().unwrap();
        let path_str = "target/debug/".to_string() + &file_name; // "target/debug/" in stead of "/target/debug/"

        // Store the crust config file in the path
        let path = Path::new(&path_str);
        let mut crust_config_path = File::create(path.clone()).unwrap();
        let file_byte = file_str.clone().into_bytes();
        crust_config_path.write_all(&file_byte).unwrap();

        /*
         *  If we run it from "cargo run --example network_reorg", it will read the config file from another path.
         *  So I create another file in that path in order for running the example.
         */
        let path_str_1 = "target/debug/examples/".to_string() + &file_name;
        let path_1 = Path::new(&path_str_1);
        let mut crust_config_example_path = File::create(path_1.clone()).unwrap();
        crust_config_example_path.write_all(&file_byte).unwrap();

        // Construct the Config object.
        let conf: Config = json::decode(&file_str).unwrap();

        BootstrapHandler {
            config: conf,
            git: git,
            file_name: p2p3_file_name
        }
    }

    fn static_info_from_their(their_info: TheirConnectionInfo) -> StaticContactInfo{
        let info_json = unwrap_result!(json::encode(&their_info));
        let data = Json::from_str(info_json.as_str()).unwrap();
        let obj = data.as_object().unwrap();
        let foo = obj.get("static_contact_info").unwrap();

        let json_str: String = foo.to_string();

        let info: StaticContactInfo = json::decode(&json_str).unwrap();
        info
    }

    pub fn update_config(&self, mp: MessagePasser) {
        let tok = mp.prepare_connection_info();
        let their_info = mp.wait_conn_info(tok);
        let mut info = BootstrapHandler::static_info_from_their(their_info);
        info.tcp_acceptors.remove(0);

        /*
         *  Because the crust can only connect to the first TCP acceptor in the config file,
         *  we need to insert the new node's info in the first position.
         */
        let mut boot_clone = self.clone();
        boot_clone.config.hard_coded_contacts[0].tcp_acceptors.insert(0, info.tcp_acceptors[0]);
        let update_str = as_pretty_json(&boot_clone.config);
        let pretty_json_str = update_str.to_string();

        // Get the p2p3 config file path and store the new config infomation it in that path.
        let path_str = self.git.local_url.clone() + &self.file_name;
        let path = Path::new(&path_str);
        let mut file = File::create(path.clone()).unwrap();
        let file_byte = pretty_json_str.into_bytes();
        file.write_all(&file_byte).unwrap();

        match self.git.commit_config("Update config file.", &self.file_name) {
            Ok(()) => (),
            Err(e) => {
                println!("Commit error: {}", e);
            }
        }

        match self.git.push() {
            Ok(()) => (),
            Err(e) => {
                println!("Push error: {}", e);
            }
        }
    }
}

/*
 *  file.suffix -> file.crust.config
 */
pub fn get_crust_config() -> Result<::std::ffi::OsString, self::crust::Error> {
    let mut name = try!(config_file_handler::exe_file_stem());
    name.push(".crust.config");
    Ok(name)
}

/*
 *  file.suffix -> file.p2p3
 */
pub fn get_p2p3_config(file_name: &String) -> String {
    let path = PathBuf::from(file_name);
    let mut name = path.file_stem().unwrap().to_os_string();
    name.push(".p2p3");
    let result = match name.into_string() {
        Ok(e) => e,
        Err(e) => {panic!("Couldn't make .p2p3 file for {} because {:?}", file_name, e);}
    };
    result
}
