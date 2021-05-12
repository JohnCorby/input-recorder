//! playback of sequences

use crate::input::{Event, Sequence};
use enigo::{Enigo, KeyboardControllable, MouseControllable};
use fps_clock::FpsClock;

pub struct Player {
    enigo: Enigo,
    fps_clock: FpsClock,
    pub sequence: Sequence,
    tick: usize,
}

impl Player {
    pub fn new(sequence: Sequence) -> Self {
        Self {
            enigo: Enigo::new(),
            fps_clock: FpsClock::new(sequence.tps as u32),
            sequence,
            tick: 0,
        }
    }

    pub fn tick(&mut self) {
        let tick = &self.sequence.ticks[self.tick];
        self.tick += 1;
        println!("{:?}", tick);

        // perform events
        // fixme mouse buttons and scrolling
        for &event in &tick.0 {
            match event {
                Event::ButtonPress(b) => self.enigo.key_down(b.into()),
                Event::ButtonRelease(b) => self.enigo.key_up(b.into()),
                Event::MousePos(x, y) => self.enigo.mouse_move_to(x as i32, y as i32),
            }
        }

        self.fps_clock.tick();
    }
}
