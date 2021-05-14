//! recording of sequences

use crate::button::Button;
use crate::input::{Event, Sequence, Tick};
use device_query::{DeviceQuery, DeviceState};
use fps_clock::FpsClock;

/// tracks kb/mouse input to turn it into events
pub struct InputState {
    device: DeviceState,

    prev_mouse_pos: (u16, u16),
    prev_buttons: Vec<Button>,
}

impl InputState {
    /// gets kb/mouse input, converting them to events
    pub fn tick(&mut self) -> Tick {
        // get state
        let mouse = self.device.get_mouse();
        let keys = self.device.get_keys();
        let mouse_pos = (mouse.coords.0 as u16, mouse.coords.1 as u16);
        let mut buttons = Button::from_mouse(&mouse);
        buttons.reserve(keys.len());
        for key in keys {
            buttons.push(key.into())
        }

        let mut events = vec![];

        // turn state into events
        if self.prev_mouse_pos != mouse_pos {
            events.push(Event::MousePos(mouse_pos.0, mouse_pos.1))
        }
        // fixme this way almost certainly sucks ass
        for &button in &buttons {
            // new button
            if !self.prev_buttons.contains(&button) {
                events.push(Event::ButtonPress(button))
            }
        }
        for &prev_button in &self.prev_buttons {
            // old button
            if !buttons.contains(&prev_button) {
                events.push(Event::ButtonRelease(prev_button))
            }
        }

        self.prev_mouse_pos = mouse_pos;
        self.prev_buttons = buttons;

        Tick(events)
    }
}

pub struct Recorder {
    input_state: InputState,
    fps_clock: FpsClock,
    pub sequence: Sequence,
}

impl Recorder {
    pub fn new(tps: u16) -> Self {
        Self {
            input_state: InputState {
                device: DeviceState::new(),
                prev_mouse_pos: (0, 0),
                prev_buttons: vec![],
            },
            fps_clock: FpsClock::new(tps as u32),
            sequence: Sequence { tps, ticks: vec![] },
        }
    }

    pub fn tick(&mut self) {
        let tick = self.input_state.tick();
        // fixme empty ticks are possible :(
        if !tick.0.is_empty() {
            println!("{:?}", tick)
        }
        self.sequence.ticks.push(tick);

        self.fps_clock.tick();
    }
}
