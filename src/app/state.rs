use super::perf::Frame;
use std::{collections::VecDeque, time::Duration};

//
// Define all state object properties
//
pub struct State {
    // Action flags, can be set to true by any consumer of the state object,
    // and will be actioned by the apply fn in app.rs
    pub actions: Actions,

    pub time_app_start: Duration,
    pub frames: VecDeque<Frame>,
    pub frames_per_second: u32,
    pub avg_frame_time: f32,
    pub monitor_refresh_rate: u32,
    //
    // Other stuff coming soon
    //
}

pub struct Actions {
    pub app_exit: bool,
    pub window_close: bool,
    pub window_open: bool,
}

//
// Create state object, & initialize with defaults
//
pub fn init() -> State {
    let actions = Actions {
        app_exit: false,
        window_close: false,
        window_open: false,
    };

    let time_app_start = Duration::from_secs(0);
    let frames_per_second = 0 as u32;
    let frames: VecDeque<Frame> = VecDeque::new();
    let avg_frame_time = 0 as f32;
    let monitor_refresh_rate = 60;

    State {
        actions,
        time_app_start,
        frames_per_second,
        frames,
        avg_frame_time,
        monitor_refresh_rate,
    }
}
