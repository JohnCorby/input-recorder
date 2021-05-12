//! tracking keyboard and mouse input

use crate::buttons::Button;
use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use fps_clock::FpsClock;
use serde::{Deserialize, Serialize};

/// holds button or mouse movement events
#[derive(Debug)]
pub enum Event {
    ButtonPress(Button),
    ButtonRelease(Button),
    MousePos(u16, u16),
}

/// stores sequence of events that you can play
#[derive(Debug)]
pub struct Sequence(pub Vec<Event>);

pub struct InputState {
    device: DeviceState,

    prev_mouse_pos: (u16, u16),
    prev_buttons: Vec<Button>,
}

impl InputState {
    /// gets kb/mouse input, converting them to events
    pub fn tick(&mut self) -> Vec<Event> {
        let mouse = self.device.get_mouse();
        let keys = self.device.get_keys();
        dbg!(&mouse, &keys);
        let mouse_pos = (mouse.coords.0 as u16, mouse.coords.1 as u16);
        let mut buttons = Button::from_mouse(mouse);
        buttons.reserve(keys.len());
        for key in keys {
            buttons.push(key.into())
        }

        let mut events = vec![];

        if self.prev_mouse_pos != mouse_pos {
            events.push(Event::MousePos(mouse_pos.0, mouse_pos.1))
        }
        // fixme this way probably sucks ass

        for (prev_button, button) in self.prev_buttons.iter().zip(buttons.iter()) {}

        self.prev_mouse_pos = mouse_pos;
        self.prev_buttons = buttons;

        events
    }
}

pub struct Recorder {
    input_state: InputState,
    fps_clock: FpsClock,
    sequence: Sequence,
}

impl Recorder {
    pub fn new(tps: u16) -> Self {
        Self {
            input_state: InputState {
                device: DeviceState::new(),
                mouse: Default::default(),
                keys: Default::default(),
            },
            fps_clock: FpsClock::new(tps as u32),
            sequence: Sequence(vec![]),
        }
    }

    pub fn tick(&mut self) {
        println!("tick!");

        self.input_state.tick();

        self.fps_clock.tick();
    }
}
