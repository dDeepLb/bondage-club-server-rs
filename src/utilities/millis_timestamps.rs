use std::time::{SystemTime, UNIX_EPOCH};

pub trait SystemTimeMillisTimestamps {
    fn get_timestamp_in_milliseconds(&self) -> i64;
}

impl SystemTimeMillisTimestamps for SystemTime {
    fn get_timestamp_in_milliseconds(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0) as i64
    }
}
