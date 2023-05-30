use super::state::State;
use super::utils::now;

use std::time::Duration;
use std::{f32, u32};

const MAX_FRAME_CACHE: usize = 300;

pub struct Frame {
    pub start: Duration,
    pub stop: Duration,
}

pub fn start_app(state: &mut State) {
    state.perf.start_time = now();
}

pub fn start_frame(state: &mut State) {
    // Create a new frame object
    let n = now();
    let f = Frame {
        start: n,
        stop: Duration::new(0, 0),
    };

    // Add to the list
    state.perf.frames.push_back(f);

    // Ensure the list is below max length, removing oldest elements
    while state.perf.frames.len() >= MAX_FRAME_CACHE {
        state.perf.frames.pop_front();
    }
}

pub fn finish_frame(state: &mut State) {
    // Update the current frame's completion time
    let n = now();
    let i = state.perf.frames.len();
    state.perf.frames[i - 1].stop = n;

    // Calc the number of frames rendered within the past 1 second,
    // while also averaging he render times of all frames in the past second
    let mut count = 0 as u32;
    let mut sum = 0 as f32;
    for f in &state.perf.frames {
        let diff = n.checked_sub(f.stop);
        let ft = f.stop.checked_sub(f.start).unwrap_or_default();
        match diff {
            Some(diff) => {
                if diff.as_secs() < 1 {
                    count += 1;
                    sum += ft.as_nanos() as f32 / 1e6;
                }
            }
            _ => {}
        }
    }
    count = std::cmp::min(1, count); // dont divide by zero
    let avg = sum / count as f32;

    // Update state objects
    state.perf.avg_frame_time = avg;

    // This likes to flicker between 60/61 fps, keep it sane, limited
    // to the monitor refresh rate
    state.perf.fps = std::cmp::min(count, state.perf.monitor_refresh_rate);
}
