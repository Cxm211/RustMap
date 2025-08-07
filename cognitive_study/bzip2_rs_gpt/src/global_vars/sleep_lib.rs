use std::time::Duration;
use std::thread::sleep;

pub fn my_sleep(secs: u64) {
    sleep(Duration::from_secs(secs));
}