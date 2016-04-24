#![allow(dead_code,unused_variables,unused_imports,unused_must_use)]
use rustc_serialize::json;
use std::io::Result as IoRes;
use std::process::{Child, Stdio};
use std::result::Result as Res;
use std::sync::mpsc::channel;
use std::sync::{mpsc, Arc, Mutex, Once, ONCE_INIT};
use std::{thread, mem};
use url::Url;
use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};
use ws::util::Token;

pub fn open_url(url: &str) -> IoRes<Child> {
    let (browser, args) = if cfg!(target_os = "linux") {
        ("xdg-open", vec![])
    } else if cfg!(target_os = "macos") {
        ("open", vec!["-g"])
    } else if cfg!(target_os = "windows") {
        // `start` requires an empty string as its first parameter.
        ("cmd", vec!["/c","start"])
    } else {
        panic!("unsupported OS")
    };
    open_specific(url, &browser, &args)
}

fn open_specific(url: &str, browser: &str, browser_args: &[&str]) -> IoRes<Child> {
    use std::process::Command;
    let url = Url::parse(url).unwrap();
    print!("starting process '{}' with url {:?}\n", browser, url);

    Command::new(browser)
        .args(browser_args)
        .arg(url.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

#[derive(Clone,RustcDecodable,RustcEncodable)]
pub enum Command{
    InsertString(usize, String),
    InsertChar(usize, char),
    DeleteChar(usize),
    Output(String),
    Commit,
    Compile,
    DisableEditing(String),
    Mode(String),    
}

pub type FnCommand = Box<Fn(&Command)->Res<String, String> + Send + Sync>;

#[allow(dead_code)]
#[derive(Clone)]
pub struct UiHandler{
    tx: mpsc::Sender<Command>,
    listeners: Arc<Mutex<Vec<FnCommand>>>,
}

pub struct UiInner{
    rx: mpsc::Receiver<Command>,
    out: Sender,
    share: UiHandler
}

impl Handler for UiInner {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.timeout(50, Token(0)).unwrap();
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(txt) => {
                let cmd: Command = json::decode(&txt).unwrap();
                for listener in self.share.listeners.lock().unwrap().iter() {
                    let res = listener(&cmd);
                    match res{
                        Ok(_)=> {},
                        Err(_)=>{panic!("Oh noooo!")}
                    }
                }
                Ok(())
            },
            Message::Binary(_) => {panic!("What the heck do I do with a binary message.")}
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        loop{
            match self.rx.try_recv() {
                Ok(cmd) => {self.out.send(Message::Text(json::encode(&cmd).unwrap())).unwrap();}
                Err(_) => {break;}
            }
        }
        self.out.timeout(50, Token(0)).unwrap();
        Ok(())
    }
}

impl UiHandler{
    #[allow(dead_code)]
    pub fn new(port: u16, url: String) -> UiHandler {
        let (tx,rx) = channel();
        println!("listening on 127.0.0.1:{}",port);
        thread::spawn(move||{
            listen(format!("127.0.0.1:{}",port).as_str(),
                 |out|{
                     let (cmdtx, cmdrx) = channel::<Command>();
                     let ui = UiInner {
                         out: out,
                         rx: cmdrx,
                         share: UiHandler{
                             listeners: Arc::new(Mutex::new(vec!())),
                             tx: cmdtx.clone() } };
                     tx.send(ui.share.clone()).unwrap();
                     ui
                 }).unwrap();
        });
        let mut browser_proc = open_url(&url).unwrap();
        browser_proc.wait();
        rx.recv().unwrap()
    }

    #[allow(dead_code)]
    pub fn add_listener(&self, f: FnCommand){
        self.listeners.lock().unwrap().push(f);
    }

    #[allow(dead_code)]
    pub fn send_command(&self, cmd: Command){
        self.tx.send(cmd).unwrap();
    }
}

#[derive(Clone)]
pub struct StaticUiHandler {
    pub inner: Arc<Mutex<UiHandler>>
}

pub fn static_ui_handler(port: u16, url: String) -> StaticUiHandler {
    // Initialize it to a null value
    static mut SINGLETON: *const StaticUiHandler = 0 as *const StaticUiHandler;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = StaticUiHandler {
                inner: Arc::new((Mutex::new(UiHandler::new(port, url))))
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        return (*SINGLETON).clone();
    }
}
