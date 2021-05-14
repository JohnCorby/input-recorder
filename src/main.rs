#![windows_subsystem = "windows"]

mod button;
mod input;
mod playback;
mod record;
mod sus;
mod ui;

use crate::playback::Player;
use crate::record::Recorder;

fn main() {
    let mut rec = Recorder::new(120);
    const SECS: u16 = 10;
    for _ in 0..rec.sequence.tps * SECS {
        rec.tick();
    }

    let player = Player::new(rec.sequence);
    for _ in player {}
}
