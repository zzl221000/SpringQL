use std::cmp::max;

use chrono::Duration;

use crate::stream_engine::time::{
    duration::{event_duration::EventDuration, SpringDuration},
    timestamp::{Timestamp, MIN_TIMESTAMP},
};

/// A watermark is held by each window.
///
/// ```text
/// watermark = max(ROWTIME) - allowed_delay
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub(in crate::stream_engine::autonomous_executor) struct Watermark {
    max_rowtime: Timestamp,
    allowed_delay: EventDuration,
}

impl Watermark {
    pub(in crate::stream_engine::autonomous_executor) fn new(allowed_delay: EventDuration) -> Self {
        Self {
            max_rowtime: MIN_TIMESTAMP,
            allowed_delay,
        }
    }

    pub(in crate::stream_engine::autonomous_executor) fn as_timestamp(&self) -> Timestamp {
        self.max_rowtime
            - Duration::from_std(*self.allowed_delay.as_std())
                .expect("duration conversion from std to chrono")
    }

    pub(in crate::stream_engine::autonomous_executor) fn update(&mut self, rowtime: Timestamp) {
        self.max_rowtime = max(rowtime, self.max_rowtime);
    }
}
