use super::state::State;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{f32, u32};

const MAX_FRAME_CACHE: usize = 300;

pub struct Frame {
    pub start: Duration,
    pub stop: Duration,
}

pub fn start_app(state: &mut State) {
    state.time_app_start = now();
}

pub fn start_frame(state: &mut State) {
    // Create a new frame object
    let n = now();
    let f = Frame {
        start: n,
        stop: Duration::new(0, 0),
    };

    // Add to the list
    state.frames.push_back(f);

    // Ensure the list is below max length, removing oldest elements
    while state.frames.len() >= MAX_FRAME_CACHE {
        state.frames.pop_front();
    }
}

pub fn finish_frame(state: &mut State) {
    // Update the most recent frame's completion time
    let n = now();
    let i = state.frames.len();
    state.frames[i - 1].stop = n;
    let mut count = 0 as u32;
    let mut sum = 0 as f32;
    for f in &state.frames {
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
    let avg = sum / count as f32;
    state.frames_per_second = std::cmp::min(count, state.monitor_refresh_rate);
    state.avg_frame_time = avg;
}

fn now() -> Duration {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0));

    since_the_epoch
}
