use serde::{Deserialize, Serialize};
use std::time::Duration;

// todo save/load

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub pre_delay: Duration,
    pub ty: rdev::EventType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Sequence {
    pub events: Vec<Event>,
}
