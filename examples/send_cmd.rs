extern crate p2p3;

use p2p3::ui::{UiHandler, Command, FnCommand};
use p2p3::woot::site::Site;
use std::io::stdin;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main(){
    let url = String::from("file:///Users/Pragya/Documents/18842_Distributed_Systems/p2p3/front-end/index.html");
    let ui = UiHandler::new(4242, url);
    fn factory() -> FnCommand {
        Box::new(|comm| {
            let command = comm.clone();
            match command {
                Command::Compile => println!("Compile button pressed"),
                Command::Commit => println!("Commit invoked"),
                Command::InsertString(_,_) => {},
                Command::InsertChar(_,_) => {},
                Command::DeleteChar(_) => {},
                Command::Output(_) => {},
                Command::DisableEditing(_) => {},
            }
            println!("in command func");
            Ok("".to_string())
        })
    };
    let commandFunc = factory();
    ui.add_listener(commandFunc);
    let initial_file_content = read_file("Cargo.toml");

    // Initialize editor content
    init_editor(&initial_file_content, ui);

    // Initialize woot
    let mut site = Site::new(1);
    site.parse_given_string(&initial_file_content);

    // for i in 0..10{
    //     ui.send_command(Command::Insert(i,"Hello there".to_string()));
    // }
    println!("Connection with front-end initialized.");
    let mut x = String::new();
    stdin().read_line(&mut x).unwrap();
}

fn read_file(url: &str) -> String {
    let path = Path::new(url);
    let mut file = match File::open(&path) {
        Err(_) => panic!("could not open"),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => panic!("Could not read"),
        Ok(_) => return s,
    }
}

fn init_editor(initial_content: &str, ui: UiHandler) {
    ui.send_command(Command::InsertString(0, initial_content.to_string()));
}
