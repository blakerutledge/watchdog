use super::config;
use crate::app::osc;
use rosc;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Apps {
    watching: bool,
    valid_listeners: bool,
    watched_apps: Vec<App>,
}

pub struct App {
    name: String,
    interface: Arc<Mutex<osc::Interface>>,
    config: config::WatchedApp,
    loop_thread: Option<thread::JoinHandle<()>>,
    listener_thread: Option<thread::JoinHandle<()>>,
    watching: Arc<AtomicBool>,
}

impl App {
    pub fn new(c: &config::WatchedApp) -> Self {
        let interface = Arc::new(Mutex::new(osc::Interface::build(c).unwrap()));
        let config = c.clone();
        let name = String::from(&c.name.val);
        let watching = Arc::new(AtomicBool::new(false));
        App {
            name,
            interface,
            config,
            loop_thread: None,
            listener_thread: None,
            watching,
        }
    }

    pub fn start(&mut self) {
        self.watching.store(true, Ordering::Relaxed);

        //
        // Heartbeat Sending thread
        let interface = self.interface.clone();
        let interval: u64 = u64::from(&self.config.heartbeat_interval.val);
        let watching = Arc::clone(&self.watching);

        let t = thread::spawn(move || loop {
            if watching.load(Ordering::Relaxed) {
                {
                    //
                    // TODO read from the channel and see if we should STOP
                    let i = interface.lock().unwrap();
                    i.send_heartbeat();
                    println!("Sending heartbeat...");
                }
                thread::sleep(Duration::from_secs(interval));
            } else {
                println!("Stopping heartbeat");
                return;
            }
        });
        self.loop_thread = Some(t);

        //
        // Listener thread
        let interface = self.interface.clone();
        let watching = Arc::clone(&self.watching);

        let t = thread::spawn(move || {
            let mut buffer = [0u8; rosc::decoder::MTU];
            loop {
                if watching.load(Ordering::Relaxed) {
                    {
                        let i = interface.lock().unwrap();
                        let received_heartbeat = i.listen(&mut buffer);
                        if received_heartbeat {
                            println!("Received heartbeat for app.");
                            //
                            // TODO use channel msg to report back..?
                            //
                        }
                    }
                    thread::sleep(Duration::from_millis(100));
                } else {
                    println!("Stopping listener");
                    return;
                }
            }
        });
        self.listener_thread = Some(t);

        // println!("Start heartbeat thread from App instance");
    }

    pub fn stop(&mut self) {
        self.watching.store(false, Ordering::Relaxed);
    }
}

impl Apps {
    pub fn new() -> Self {
        Apps {
            watching: false,
            valid_listeners: false,
            watched_apps: Vec::new(),
        }
    }

    pub fn build_listeners(&mut self, config: &config::Config) {
        if self.watching {
            println!("Cannot rebuild already running listeners");
            return;
        }
        self.watching = true;

        println!(
            "Building listeners for each watched app, total of: {}",
            config.watched_apps.len()
        );

        for watched_app in config.watched_apps.iter() {
            let a = App::new(watched_app);
            self.watched_apps.push(a);
        }

        self.valid_listeners = true;

        println!("Starting apps");
        for a in self.watched_apps.iter_mut() {
            a.start();
        }
    }

    pub fn destroy_listeners(&mut self) {
        if !self.watching {
            println!("Cannot stop apps when they are not yet running");
            return;
        }
        self.watching = false;
        for a in self.watched_apps.iter_mut() {
            a.stop();
        }

        // Is this enough to drop them..?
        self.watched_apps.clear();
    }
}

pub fn init() -> Apps {
    let apps = Apps::new();

    return apps;
}
