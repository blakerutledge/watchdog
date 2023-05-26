// HIDE console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// - - - IMPORTS - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

mod gui;
use gui::Gui;

// - - - MAIN - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct WatchdogApp {
    gui: Gui,
}

impl WatchdogApp {
    fn new() -> Self {
        WatchdogApp { gui: Gui::new() }
    }

    fn start(mut self) {
        self.gui
            .event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| self.gui.update(event, control_flow))
    }
}

fn main() {
    let mut app = WatchdogApp::new();
    app.gui.init_ui();
    app.start();
}
