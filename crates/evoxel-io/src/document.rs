use chrono::{DateTime, TimeZone, Timelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvoxelInfoDocument {
    pub resolution: f64,
    pub frame_id: String,
    pub start_timestamp: Option<TimeElement>,
    pub stop_timestamp: Option<TimeElement>,
    pub submap_index: Option<i32>,
}

impl EvoxelInfoDocument {
    pub fn new(frame_id: String, resolution: f64) -> Self {
        Self {
            frame_id,
            resolution,
            start_timestamp: None,
            stop_timestamp: None,
            submap_index: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeElement {
    sec: i64,
    nanosec: u32,
}

impl From<TimeElement> for DateTime<Utc> {
    fn from(item: TimeElement) -> Self {
        Utc.timestamp_opt(item.sec, item.nanosec).unwrap()
    }
}

impl From<DateTime<Utc>> for TimeElement {
    fn from(item: DateTime<Utc>) -> Self {
        Self {
            sec: item.timestamp(),
            nanosec: item.nanosecond(),
        }
    }
}
