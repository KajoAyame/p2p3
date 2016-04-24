use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::mem;
use storage::storage_helper::GitAccess;
use compile::CompileMode;

#[derive(Clone)]
pub struct P2P3Globals {
    pub inner: Arc<Mutex<P2P3Values>>
}

#[derive(Clone)]
pub struct P2P3Values {
    site_id: u32,
    port: u16,
    url: String,
    git_access: GitAccess,
    mode: CompileMode
}

impl P2P3Values {
    pub fn init(&mut self, site_id: u32, port: u16, url: String, git_access: GitAccess) {
        self.site_id = site_id;
        self.port = port;
        self.url = url;
        self.git_access = git_access;
        self.mode = CompileMode::None;
    }

    pub fn get_site_id(&self) -> u32 {
        self.site_id
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_git_access(&self) -> GitAccess {
        self.git_access.clone()
    }

    pub fn get_compile_mode(&self) -> CompileMode {
        self.mode.clone()
    }

    pub fn set_compile_mode(&mut self, mode: CompileMode) {
        self.mode = mode;
    }
}

pub fn p2p3_globals() -> P2P3Globals {
    // Initialize it to a null value
    static mut SINGLETON: *const P2P3Globals = 0 as *const P2P3Globals;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let globals = P2P3Values {
                site_id: 0,
                port: 8080,
                url: String::new(),
                git_access: GitAccess::new(String::new(),String::new(),String::new(),String::new(),String::new()),
                mode: CompileMode::None
            };
            let singleton = P2P3Globals {
                inner: Arc::new((Mutex::new(globals)))
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        return (*SINGLETON).clone();
    }
}
