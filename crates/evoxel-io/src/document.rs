use chrono::{DateTime, TimeZone, Timelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VoxelGridInfoDocument {
    pub frame_id: String,
    pub resolution: f64,
    pub start_timestamp: TimeElement,
    pub stop_timestamp: TimeElement,
    pub submap_index: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
