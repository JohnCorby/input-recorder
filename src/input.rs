//! tracking keyboard and mouse input

use crate::button::Button;
use serde::{Deserialize, Serialize};

/// holds button or mouse movement events
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Event {
    ButtonPress(Button),
    ButtonRelease(Button),
    MousePos(u16, u16),
}

/// stores a single tick's worth of events
#[derive(Debug, Serialize, Deserialize)]
pub struct Tick(pub Vec<Event>);

/// stores sequence of ticks
#[derive(Debug, Serialize, Deserialize)]
pub struct Sequence {
    pub tps: u16,
    pub ticks: Vec<Tick>,
}
