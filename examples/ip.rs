use std::net_ip;
use std::uv;

fn main() {
    let iotask = uv::global_loop::get();
    let result = net_ip::get_addr("54.209.245.74", &iotask);

    //io::println(fmt!("%?", result));
 }
