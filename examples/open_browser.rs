extern crate p2p3;

use p2p3::ui::open_url;

fn main(){
    let mut browser_proc = open_url("http://google.com/").unwrap();
    browser_proc.wait();
}
