use chrono::{DateTime, Duration, Utc};
use ecoord::FrameId;

#[derive(Debug, Clone, PartialEq)]
pub struct VoxelGridInfo {
    pub(crate) frame_id: FrameId,
    pub(crate) resolution: f64,
    pub(crate) start_time: Option<DateTime<Utc>>,
    pub(crate) stop_time: Option<DateTime<Utc>>,
    pub(crate) submap_index: Option<i32>,
}

impl VoxelGridInfo {
    pub fn new(
        frame_id: FrameId,
        resolution: f64,
        start_time: Option<DateTime<Utc>>,
        stop_time: Option<DateTime<Utc>>,
        submap_index: Option<i32>,
    ) -> Self {
        Self {
            resolution,
            frame_id,
            start_time,
            stop_time,
            submap_index,
        }
    }

    pub fn frame_id(&self) -> &FrameId {
        &self.frame_id
    }

    pub fn resolution(&self) -> f64 {
        self.resolution
    }

    pub fn start_time(&self) -> &Option<DateTime<Utc>> {
        &self.start_time
    }

    pub fn stop_time(&self) -> &Option<DateTime<Utc>> {
        &self.stop_time
    }

    pub fn submap_index(&self) -> Option<i32> {
        self.submap_index
    }

    pub fn duration(&self) -> Option<Duration> {
        if self.start_time.is_some() && self.stop_time.is_some() {
            Some(self.stop_time.unwrap() - self.start_time.unwrap())
        } else {
            None
        }
    }
}
