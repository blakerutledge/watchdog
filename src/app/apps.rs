pub struct Apps {
    watching: bool,
    valid_listeners: bool,
}

use super::config::Config;

impl Apps {
    pub fn new() -> Self {
        Apps {
            watching: false,
            valid_listeners: false,
        }
    }

    pub fn build_listeners(&mut self, config: &Config) {
        if self.watching {
            println!("Cannot rebuild active listeners");
            return;
        }

        if self.valid_listeners {
            println!("Cannot build listeners, they are already built");
            return;
        }

        println!(
            "TODO: build {:?} listeners according to config",
            config.watched_apps.len()
        );

        self.valid_listeners = true;
    }

    pub fn destroy_listeners(&mut self) {
        if !self.valid_listeners {
            println!("Cannot destroy listeners, they are not yet built");
            return;
        }

        println!("TODO: destroy listeners");

        self.valid_listeners = false;

        if self.watching {
            self.watching = false;

            println!("Abandoning already running apps");
        }
    }

    pub fn start_apps(&mut self) {
        if !self.valid_listeners {
            println!("Cannot start apps without valid listeners");
            return;
        }

        self.watching = true;
        println!("TODO: start apps");
    }

    pub fn stop_apps(&mut self) {
        if !self.watching {
            println!("Cannot stop apps when they are not yet running");
            return;
        }

        self.watching = false;
        println!("TODO: stop apps");
    }
}

pub fn init() -> Apps {
    let apps = Apps::new();

    return apps;
}
