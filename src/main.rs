#![allow(dead_code,unused_variables,unused_imports)]

extern crate crust;
extern crate time;
extern crate git2;
#[macro_use]
extern crate maidsafe_utilities;
extern crate rustc_serialize;
extern crate docopt;
extern crate rand;
extern crate getopts;
extern crate ws;
extern crate url;

mod commit;
mod compile;
mod logger;
pub mod network;
mod permission;
pub mod storage;
mod ui;
mod woot;
mod utils;
mod async_queue;

use std::{thread,env};
use getopts::Options;
use storage::storage_helper::GitAccess;
use woot::static_site::site_singleton;
use woot::operation_thread::run;
use permission::permissions_handler::get_permission_level;
use permission::permissions_handler::PermissionLevel;
use compile::{CompileMode, run_code};
use ui::{UiHandler, Command, FnCommand, open_url, static_ui_handler};
use utils::p2p3_globals;
use std::io::stdin;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("u", "", "URL to the git repo to connect to", "URL");
    opts.optopt("n", "", "Username", "Username");
    opts.optopt("p", "", "Password", "Password");
    opts.optopt("s", "", "Site id", "SiteId");
    opts.optopt("f", "", "File path to clone the git repo", "FilePath");
    opts.optopt("d", "port", "Port number", "PortNumber");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    };
    let git_url = matches.opt_str("u").unwrap();
    let git_username = matches.opt_str("n").unwrap();
    let git_password = matches.opt_str("p").unwrap();
    let site_id_str = matches.opt_str("s").unwrap();
    let site_id = site_id_str.parse::<u32>().unwrap();
    let port = matches.opt_str("d").unwrap();
    let port_number = port.parse::<u16>().unwrap();
    let local_path = matches.opt_str("f").unwrap();
    let p = env::current_dir().unwrap();
    let p2p3_url = format!("file://{}/front-end/index.html",p.display());
    let file_path = "c_code.c";
    let git_access = GitAccess::new(git_url.clone(), local_path.clone(), file_path.to_string().clone(), git_username.clone(), git_password.clone());

    {
        let globals = p2p3_globals().inner.clone();
        let mut values = globals.lock().unwrap();
        values.init(site_id, port_number, p2p3_url.clone(), git_access.clone());
    }

    if matches.free.len() > 0 {
        print_usage(&program, opts);
        return;
    };
    let static_site = site_singleton(site_id);
    match git_access.clone_repo() {
        Ok(()) => {},
        Err(e) => {
            println!("The folder already exits");
        },
    };

    let permission_level = get_permission_level(&git_access);
    match permission_level {
        PermissionLevel::Editor => println!("The user is an editor"),
        PermissionLevel::Viewer => println!("The user is a viewer"),
    };
    let operation_thread = thread::spawn(move || {
        run(site_id);
    });
    let file_name = &(local_path+file_path);
    {
        let initial_file_content = read_file(file_name);
        let site_clone = static_site.inner.clone();
        let mut site = site_clone.lock().unwrap();
        site.parse_given_string(&initial_file_content);
    }

    let static_ui = static_ui_handler(port_number, p2p3_url.clone());
    println!("Called Static UI Handler");
    fn recieve_commands() -> FnCommand {
        Box::new(|comm| {
            let command = comm.clone();
            match command {
                Command::Compile => {
                    let globals = p2p3_globals().inner.clone();
                    let values = globals.lock().unwrap();
                    let site_id = values.get_site_id();
                    let site_clone = site_singleton(site_id).inner.clone();
                    let mut site = site_clone.lock().unwrap();
                    let ui_clone = static_ui_handler(values.get_port(), values.get_url()).inner.clone();
                    let ui = ui_clone.lock().unwrap();
                    match run_code(values.get_compile_mode(), &site.content()) {
                        Ok(o) => ui.send_command(Command::Output(o)),
                        Err(e) => println!("error {}", e),
                    };
                },
                Command::InsertChar(position, character) => {
                    println!("Received {} {}", position, character);
                    let globals = p2p3_globals().inner.clone();
                    let values = globals.lock().unwrap();
                    let site_clone = site_singleton(values.get_site_id()).inner.clone();
                    let mut site = site_clone.lock().unwrap();
                    site.generate_insert(position, character, true);
                    println!("Site content {}", site.content());
                },
                Command::DeleteChar(position) => {
                    println!("Received {}", position);
                    let globals = p2p3_globals().inner.clone();
                    let values = globals.lock().unwrap();
                    let site_clone = site_singleton(values.get_site_id()).inner.clone();
                    let mut site = site_clone.lock().unwrap();
                    site.generate_del(position);
                    println!("Site content {}", site.content());
                },
                Command::Commit => {
                    let globals = p2p3_globals().inner.clone();
                    let values = globals.lock().unwrap();
                    let ga = values.get_git_access();
                    ga.commit_path("Commit message").unwrap();
                    ga.push().unwrap();
                },
                Command::InsertString(position, content) => {

                },
                Command::Output(results) => {

                },
                Command::DisableEditing(_) => {

                },
                Command::Mode(mode) => {
                    println!("Mode selected: {}", mode);
                    let globals = p2p3_globals().inner.clone();
                    let mut values = globals.lock().unwrap();
                    values.set_compile_mode(mode.parse::<CompileMode>().unwrap());
                },
            }
            Ok("".to_string())
        })
    };
    println!("Created receive commands");
    let command_func = recieve_commands();
    {
        let ui_inner = static_ui.inner.clone();
        let ui = ui_inner.lock().unwrap();

        ui.add_listener(command_func);
        let mut content = String::new();
        {
            let site_clone = static_site.inner.clone();
            let mut site = site_clone.lock().unwrap();
            let mut borrowed_content = &mut content;
            *borrowed_content = site.content();
        }
        ui.send_command(Command::InsertString(0, content));
        match permission_level {
            PermissionLevel::Editor => {},
            PermissionLevel::Viewer => {
                ui.send_command(Command::DisableEditing(String::new()))
            },
        };
    }
    println!("Connection with front-end initialized.");
    let mut x = String::new();
    stdin().read_line(&mut x).unwrap();
}

fn read_file(url: &str) -> String {
    let path = Path::new(url);
    let mut file = match File::open(&path) {
        Err(_) => panic!("could not open {}", url),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => panic!("Could not read"),
        Ok(_) => return s,
    }
}
