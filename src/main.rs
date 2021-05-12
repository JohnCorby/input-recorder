#![windows_subsystem = "windows"]

mod button;
mod input;
mod playback;
mod record;
mod sus;
mod ui;

use crate::record::Recorder;

fn main() {
    let mut rec = Recorder::new(60);
    loop {
        rec.tick();
    }
}
