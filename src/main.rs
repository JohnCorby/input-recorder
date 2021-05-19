#![windows_subsystem = "windows"]

mod data;
mod global_comm;
mod input;

use data::Sequence;
use iced::{
    button, Align, Application, Button, Checkbox, Clipboard, Column, Command, Container, Element,
    Length, Settings, Space, Subscription, Text,
};
use nfd::Response;
use std::path::PathBuf;

fn main() {
    // dbg!(std::mem::size_of::<data::Event>());
    show();
}

pub fn show() {
    App::run(Settings::default()).unwrap()
}

#[derive(Debug)]
struct App {
    recording: bool,
    playing: bool,
    looping: bool,
    current_seq: Option<Sequence>,
    current_file: Option<PathBuf>,
    // seq_index: Option<usize>,
    dirty: bool,

    rec_button: button::State,
    play_button: button::State,
    save_button: button::State,
    load_button: button::State,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    RecStart,
    RecStop,
    PlayStart,
    PlayStop,
    Loop(bool),
    Save,
    Load,
    InputEvent(data::Event),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        global_comm::init();
        input::init();

        (
            Self {
                recording: false,
                playing: false,
                looping: false,
                current_seq: None,
                current_file: None,
                // seq_index: None,
                dirty: false,

                rec_button: Default::default(),
                play_button: Default::default(),
                save_button: Default::default(),
                load_button: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!(
            "Input Recorder - {}{:?}",
            if self.dirty { "*" } else { "" },
            self.current_file
        )
    }

    fn update(&mut self, message: Message, _: &mut Clipboard) -> Command<Message> {
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

        global_comm::outgoing(message);

        use Message::*;
        match message {
            RecStart => {
                self.playing = false;
                self.recording = true;

                self.current_seq = Some(Sequence::default());
                self.dirty = true;
            }
            RecStop => self.recording = false,

            PlayStart => {
                self.recording = false;

                if let Some(seq) = &self.current_seq {
                    self.playing = true;

                    return Command::perform(input::play(seq.clone(), self.looping), |_| {
                        Message::PlayStop
                    });
                }
            }
            PlayStop => self.playing = false,
            Loop(looping) => self.looping = looping,

            Save => {
                self.recording = false;
                self.playing = false;
                if let Some(seq) = &self.current_seq {
                    if let Some(path) = pick_file(true) {
                        data::save(seq, &path);
                        self.current_file = Some(path);
                        self.dirty = false;
                    }
                }
            }
            Load => {
                self.recording = false;
                self.playing = false;
                if let Some(path) = pick_file(false) {
                    self.current_seq = Some(data::load(&path));
                    self.current_file = Some(path);
                }
            }

            InputEvent(event) => {
                if self.recording {
                    if let Some(seq) = &mut self.current_seq {
                        seq.events.push(event)
                    }
                }
            }
        };
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        global_comm::incoming()
    }

    fn view(&mut self) -> Element<Message> {
        let rec = Button::new(
            &mut self.rec_button,
            Text::new(format!(
                "{} Recording",
                if !self.recording { "Start" } else { "Stop" }
            )),
        )
        .on_press(match self.recording {
            false => Message::RecStart,
            true => Message::RecStop,
        });
        let play = Button::new(
            &mut self.play_button,
            Text::new(format!(
                "{} Playing",
                if !self.playing { "Start" } else { "Stop" }
            )),
        )
        .on_press(match self.playing {
            false => Message::PlayStart,
            true => Message::PlayStop,
        });

        let content = Column::new()
            .align_items(Align::Center)
            .push(Text::new("Input Recorder of Piss"))
            .push(Space::with_height(Length::Fill))
            .push(rec)
            .push(play)
            .push(Checkbox::new(self.looping, "Loop", Message::Loop))
            .push(Space::with_height(Length::Fill))
            .push(Text::new(format!("Current File: {:?}", self.current_file)))
            .push(Button::new(&mut self.save_button, Text::new("Save")).on_press(Message::Save))
            .push(Button::new(&mut self.load_button, Text::new("Load")).on_press(Message::Load));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
