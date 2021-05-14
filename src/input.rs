//! tracking keyboard and mouse input

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// holds button or mouse movement events
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub pre_delay: Duration,
    pub ty: rdev::EventType,
}

/// stores sequence of ticks
#[derive(Debug, Serialize, Deserialize)]
pub struct Sequence {
    pub events: Vec<Event>,
}
