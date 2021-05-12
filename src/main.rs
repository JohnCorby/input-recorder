#![windows_subsystem = "windows"]
#![allow(unused)]

mod buttons;
mod input;
mod sus;
mod ui;

use crate::input::{Recorder, Sequence};
use device_query::DeviceState;
use fps_clock::FpsClock;

fn main() {
    let mut rec = Recorder::new(1);
    loop {
        rec.tick();
    }
}
