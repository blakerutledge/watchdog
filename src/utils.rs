use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn now() -> Duration {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0));

    since_the_epoch
}
