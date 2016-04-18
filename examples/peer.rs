extern crate p2p3;
extern crate crust;
#[macro_use]
extern crate maidsafe_utilities; // macro unwrap!()

use crust::{Service, OurConnectionInfo, Event, ConnectionInfoResult, read_config_file, write_config_file};
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::str;
//use crust::service::{prepare_connection_info, to_their_connection_info};

fn main() {
    ///////
    let (event_tx_0, event_rx_0) = channel();
    let (category_tx_0, category_rx_0) = channel();

    //    let (bs_sender, bs_receiver) = channel();
    let event_category_0 =
        ::maidsafe_utilities::event_sender::MaidSafeEventCategory::Crust;
    let event_sender_0 =
        ::maidsafe_utilities::event_sender::MaidSafeObserver::new(event_tx_0,
                                                                  event_category_0,
                                                                  category_tx_0);

    let (event_tx_1, event_rx_1) = channel();
    let (category_tx_1, category_rx_1) = channel();

    //    let (bs_sender, bs_receiver) = channel();
    let event_category_1 =
        ::maidsafe_utilities::event_sender::MaidSafeEventCategory::Crust;
    let event_sender_1 =
        ::maidsafe_utilities::event_sender::MaidSafeObserver::new(event_tx_1,
                                                                  event_category_1,
                                                                  category_tx_1);

    //let (event_sender_0, _category_rx_0, event_rx_0) = get_event_sender();
    //let (event_sender_1, _category_rx_1, event_rx_1) = get_event_sender();
    ///////
    let mut config = read_config_file().unwrap();
    config.enable_utp = true;
    config.enable_tcp = true;

    let mut service_0 = unwrap_result!(Service::with_config(event_sender_0, &config));
    service_0.start_listening_tcp();

    match unwrap_result!(event_rx_0.recv()) {
            Event::BootstrapFinished => (),
            event => panic!("Received unexpected event: {:?}", event),
    }

    let mut service_1 = unwrap_result!(Service::with_config(event_sender_1, &config));
    service_1.start_listening_tcp();

    match unwrap_result!(event_rx_1.recv()) {
           Event::BootstrapFinished => (),
           event => panic!("Received unexpected event: {:?}", event),
    }

    let our_ci_0 = prepare_connection_info(&mut service_0, &event_rx_0);
    let their_ci_0 = our_ci_0.to_their_connection_info();

    let our_ci_1 = prepare_connection_info(&mut service_1, &event_rx_1);
    let their_ci_1 = our_ci_1.to_their_connection_info();

    service_0.connect(our_ci_0, their_ci_1);
    service_1.connect(our_ci_1, their_ci_0);

    let id_1 = match unwrap_result!(event_rx_0.recv()) {
        Event::NewPeer(Ok(()), their_id) => their_id,
        m => panic!("0 Should have connected to 1. Got message {:?}", m),
    };
    println!("id_1 = {}", id_1);

    let id_0 = match unwrap_result!(event_rx_1.recv()) {
        Event::NewPeer(Ok(()), their_id) => their_id,
        m => panic!("1 Should have connected to 0. Got message {:?}", m),
    };
    println!("id_0 = {}", id_0);


        // send data from 0 to 1
        {
            let data_txd = vec![0, 1, 255, 254, 222, 1];
            unwrap_result!(service_0.send(&id_1, data_txd.clone()));

            // 1 should rx data
                let event_rxd = unwrap_result!(event_rx_1.recv());
                match event_rxd {
                    Event::NewMessage(their_id, msg) => {
                        println!("receive msg {}",  String::from_utf8(msg).unwrap());
                    }
                    _ => panic!("Received unexpected event: {:?}", event_rxd),
                }

        }

        // send data from 1 to 0
              {
                  let data_txd = vec![10, 11, 155, 214, 202];
                  unwrap_result!(service_1.send(&id_0, data_txd.clone()));

                  // 0 should rx data
                  let (data_rxd, peer_id) = {
                      let event_rxd = unwrap_result!(event_rx_0.recv());
                      match event_rxd {
                          Event::NewMessage(their_id, msg) => (msg, their_id),
                          _ => panic!("Received unexpected event: {:?}", event_rxd),
                      }
                  };

                  assert_eq!(data_rxd, data_txd);
                  assert_eq!(peer_id, id_1);
              }
    /*
    match unwrap_result!(event_rx_1.recv()) {
           Event::BootstrapFinished => (),
           event => panic!("Received unexpected event: {:?}", event),
    }

    let our_ci_0 = Service::prepare_connection_info(&mut service_0, &event_rx_0);
    let their_ci_0 = our_ci_0.to_their_connection_info();

    let our_ci_1 = Service::prepare_connection_info(&mut service_1, &event_rx_1);
    let their_ci_1 = our_ci_1.to_their_connection_info();
*/
}

fn prepare_connection_info(service: &mut Service, event_rx: &Receiver<Event>) -> OurConnectionInfo {
     //static TOKEN_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
     //let token = TOKEN_COUNTER.fetch_add(1, Ordering::Relaxed) as u32;
     let token = 0;
     service.prepare_connection_info(token);

     match unwrap_result!(event_rx.recv()) {
         Event::ConnectionInfoPrepared(cir) => {
             assert_eq!(cir.result_token, token);
             unwrap_result!(cir.result)
         }
         event => panic!("Received unexpected event: {:?}", event),
     }
}

/*
fn get_event_sender()
        -> (::CrustEventSender,
            Receiver<MaidSafeEventCategory>,
            Receiver<Event>)
    {
        let (category_tx, category_rx) = mpsc::channel();
        let event_category = MaidSafeEventCategory::Crust;
        let (event_tx, event_rx) = mpsc::channel();

        (MaidSafeObserver::new(event_tx, event_category, category_tx),
         category_rx,
         event_rx)
    }
    */
