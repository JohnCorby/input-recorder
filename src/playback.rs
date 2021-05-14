//! playback of sequences

use crate::input::{Event, Sequence};
use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use fps_clock::FpsClock;
use std::convert::TryFrom;

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
        if !tick.0.is_empty() {
            println!("{:?}", tick)
        }

        // perform events
        // fixme mouse buttons and scrolling
        for &event in &tick.0 {
            // fixme this is the worst
            match event {
                Event::ButtonPress(b) => {
                    if let Ok(key) = Key::try_from(b) {
                        self.enigo.key_down(key)
                    } else if let Ok(button) = MouseButton::try_from(b) {
                        self.enigo.mouse_down(button)
                    } else {
                        todo!("convert {:?} into enigo", b)
                    }
                }
                Event::ButtonRelease(b) => {
                    if let Ok(key) = Key::try_from(b) {
                        self.enigo.key_up(key)
                    } else if let Ok(button) = MouseButton::try_from(b) {
                        self.enigo.mouse_up(button)
                    } else {
                        todo!("convert {:?} into enigo", b)
                    }
                }
                Event::MousePos(x, y) => self.enigo.mouse_move_to(x as i32, y as i32),
            }
        }

        self.fps_clock.tick();
    }
}

/// bs easy method of playback
impl Iterator for Player {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        if self.tick == self.sequence.ticks.len() {
            return None;
        }
        self.tick();
        Some(())
    }
}
