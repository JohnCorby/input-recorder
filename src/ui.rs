use std::path::PathBuf;

use iced::{
    button, Align, Button, Checkbox, Column, Element, Length, Sandbox, Settings, Space, Text,
};
use nfd::Response;

use crate::data::Sequence;

pub fn show() {
    App::run(Settings::default()).unwrap()
}

#[derive(Debug)]
struct App {
    recording: bool,
    playing: bool,
    looping: bool,
    current_seq: Option<Sequence>,
    seq_index: Option<usize>,

    rec_button: button::State,
    play_button: button::State,
    save_button: button::State,
    load_button: button::State,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            recording: false,
            playing: false,
            looping: false,
            current_seq: None,
            seq_index: None,

            rec_button: Default::default(),
            play_button: Default::default(),
            save_button: Default::default(),
            load_button: Default::default(),
        }
    }

    fn title(&self) -> String {
        "Input Recorder".to_string()
    }

    fn update(&mut self, message: Message) {
        fn pick_file(save: bool) -> Option<PathBuf> {
            const FILE_EXT: &str = "irs";
            use nfd::DialogType::*;
            let res = nfd::open_dialog(
                Some(FILE_EXT),
                Some(std::env::current_dir().unwrap().to_str().unwrap()),
                if save { SaveFile } else { SingleFile },
            )
            .unwrap();
            match res {
                Response::Okay(path) => Some(path.into()),
                Response::OkayMultiple(_) => unreachable!(),
                Response::Cancel => None,
            }
        }

        match message {
            Message::RecStart => self.recording = true,
            Message::RecStop => self.recording = false,
            Message::PlayStart => self.playing = true,
            Message::PlayStop => self.playing = false,
            Message::Loop(looping) => self.looping = looping,
            Message::Save => {
                if let Some(seq) = self.current_seq.as_ref() {
                    if let Some(path) = pick_file(true) {
                        crate::data::save(seq, &path)
                    }
                }
            }
            Message::Load => {
                if let Some(path) = pick_file(false) {
                    self.current_seq = Some(crate::data::load(&path));
                }
            }
        };
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new("Input Recorder of Piss"))
            .push(Space::with_height(Length::Fill))
            .push(
                Button::new(
                    &mut self.rec_button,
                    Text::new(format!(
                        "{} Recording",
                        if !self.recording { "Start" } else { "Stop" }
                    )),
                )
                .on_press(match self.recording {
                    false => Message::RecStart,
                    true => Message::RecStop,
                }),
            )
            .push(
                Button::new(
                    &mut self.play_button,
                    Text::new(format!(
                        "{} Playing",
                        if !self.playing { "Start" } else { "Stop" }
                    )),
                )
                .on_press(match self.playing {
                    false => Message::PlayStart,
                    true => Message::PlayStop,
                }),
            )
            .push(Checkbox::new(self.looping, "Loop", |checked| {
                Message::Loop(checked)
            }))
            .push(Space::with_height(Length::Fill))
            .push(Text::new(format!(
                "Current File: {:?}",
                self.current_seq.as_ref().and_then(|seq| seq.file.as_ref())
            )))
            .push(Button::new(&mut self.save_button, Text::new("Save")).on_press(Message::Save))
            .push(Button::new(&mut self.load_button, Text::new("Load")).on_press(Message::Load))
            .width(Length::Fill)
            .align_items(Align::Center)
            .into()
    }
}

#[derive(Debug, Copy, Clone)]
enum Message {
    RecStart,
    RecStop,
    PlayStart,
    PlayStop,
    Loop(bool),
    Save,
    Load,
}
