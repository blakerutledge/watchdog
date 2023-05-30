use super::perf::Frame;
use std::{collections::VecDeque, time::Duration};

//
// Define all state object properties, nested into categories
//
pub struct State {
    // Action flags, can be set to true by any consumer of the state object,
    // and will be actioned by the apply fn in app.rs
    pub actions: Actions,

    // Stats for host Window rendering
    pub perf: Perf,
    // Watched App heartbeats
}

pub struct Actions {
    pub app_exit: bool,
    pub window_close: bool,
    pub window_open: bool,
}

pub struct Perf {
    pub start_time: Duration,
    pub frames: VecDeque<Frame>,
    pub fps: u32,
    pub avg_frame_time: f32,
    pub monitor_refresh_rate: u32,
}

//
// Create state object, & initialize with defaults
//
pub fn init() -> State {
    //
    // all action flags are initialized as false
    let actions = Actions {
        app_exit: false,
        window_close: false,
        window_open: false,
    };

    let perf = Perf {
        start_time: Duration::from_secs(0),
        fps: 0 as u32,
        frames: VecDeque::new(),
        avg_frame_time: 0 as f32,
        monitor_refresh_rate: 60,
    };

    State { actions, perf }
}
