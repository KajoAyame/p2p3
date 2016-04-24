#![allow(dead_code,unused_variables,unused_imports)]

use std::sync::{Arc,Mutex,Once,ONCE_INIT,Condvar};
use std::{mem,thread,env};
use super::site::Site;
use super::operation::Operation;

#[derive(Clone)]
pub struct StaticSite {
    pub inner: Arc<Mutex<Site>>
}

pub fn site_singleton(site_id: u32) -> StaticSite {
    // Initialize it to a null value
    static mut SINGLETON: *const StaticSite = 0 as *const StaticSite;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = StaticSite {
                inner: Arc::new((Mutex::new(Site::new(site_id))))
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));

            // Make sure to free heap memory at exit
            /* This doesn't exist in stable 1.0, so we will just leak it!
            rt::at_exit(|| {
                let singleton: Box<StaticSite> = mem::transmute(SINGLETON);

                // Let's explictly free the memory for this example
                drop(singleton);

                // Set it to null again. I hope only one thread can call `at_exit`!
                SINGLETON = 0 as *const _;
            });
            */
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        return (*SINGLETON).clone();
    }
}
