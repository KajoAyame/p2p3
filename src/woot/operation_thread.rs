#![allow(dead_code,unused_variables,unused_imports)]
use super::static_site::site_singleton;
use rustc_serialize::json;
use super::char_id::create_char_id;
use super::char_id::CharId;
use super::woot_char::WootChar;
use super::operation::Operation;
use std::{thread,env};
use std::thread::sleep;
use std::time::Duration;
use std::sync::mpsc::channel;

pub fn run(site_id: u32) {
    let static_site = site_singleton(site_id);

    let site = static_site.inner.clone();
    let site2 = static_site.inner.clone();

    // Implementing Multiple Producer, Single Consumer
    let (sender, receiver) = channel();
    thread::spawn(move || {
        loop {
            let mut site = site.lock().unwrap();
            // Receive operation
            match site.pool.pop_front() {
                Some(v) => {
                    println!("Found something!");
                    sender.send(v).unwrap();
                },
                None => {}
            }
        }
    });

    loop {
        let op = receiver.recv().unwrap();
        let mut site2 = site2.lock().unwrap();
        site2.implement_operation(op);
        println!("Implemented something");
    }
}

//#[test]
fn test_run() {
    let char_id_1 = create_char_id(1, 0);
    let char_id_2 = create_char_id(2, 0);
    let mut wchar1 = WootChar::new(char_id_1.clone(), 'a', CharId::Beginning, CharId::Ending); // From site 1
    let mut wchar2 = WootChar::new(char_id_2.clone(), 'b', char_id_1.clone(), CharId::Ending); // From site 2
    let operation1 = Operation::Insert{w_char: wchar1.clone(), from_site: 1};
    let operation2 = Operation::Insert{w_char: wchar2.clone(), from_site: 1};
    let mut static_site = site_singleton(0);
    {
        let site = static_site.inner.clone();
        let mut site = site.lock().unwrap();
        let encoded1 = json::encode(&operation1).unwrap();
        let encoded2 = json::encode(&operation2).unwrap();
        site.reception(encoded1);
        site.reception(encoded2);
        assert_eq!(site.pool.len(), 2);
    }
    run(0);
    {
        let site = static_site.inner.clone();
        let mut site = site.lock().unwrap();
        assert_eq!(site.pool.len(), 0);
    }
}
