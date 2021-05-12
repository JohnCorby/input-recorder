//! tracking keyboard and mouse input

use crate::buttons::Button;
// use serde::{Deserialize, Serialize};

/// holds button or mouse movement events
#[derive(Debug, Copy, Clone)]
pub enum Event {
    ButtonPress(Button),
    ButtonRelease(Button),
    MousePos(u16, u16),
}

/// stores a single tick's worth of events
#[derive(Debug)]
pub struct Tick(pub Box<[Event]>);

/// stores sequence of ticks
#[derive(Debug)]
pub struct Sequence(pub Vec<Tick>);
