#![feature(once_cell)]
#![windows_subsystem = "windows"]

mod channel;
mod data;
mod input;

use crate::channel::Channel;
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

    seq: Option<Sequence>,
    file: Option<PathBuf>,
    seq_index: usize,

    dirty: bool,

    rec_button: button::State,
    play_button: button::State,
    save_button: button::State,
    load_button: button::State,

    channel: Channel,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    RecStart,
    InputReceived(data::Event),
    RecStop,
    PlayStart,
    PlayNext,
    PlayStop,
    Loop(bool),
    Save,
    Load,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        let channel = Channel::new();
        input::init_listener(channel.tx.clone());

        (
            Self {
                recording: false,
                playing: false,
                looping: false,

                seq: None,
                file: None,
                seq_index: 0,

                dirty: false,

                rec_button: Default::default(),
                play_button: Default::default(),
                save_button: Default::default(),
                load_button: Default::default(),

                channel,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!(
            "Input Recorder - {}{:?}",
            if self.dirty { "*" } else { "" },
            self.file
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

        // println!("m {:?}", message);
        use Message::*;
        match message {
            RecStart => {
                self.playing = false;
                self.recording = true;

                self.seq = Some(Sequence::default());
                self.dirty = true;
            }
            InputReceived(event) => {
                if self.recording {
                    if let Some(seq) = &mut self.seq {
                        seq.events.push(event)
                    }
                }
            }
            RecStop => self.recording = false,

            PlayStart => {
                self.recording = false;

                if self.seq.is_some() {
                    self.playing = true;
                    self.seq_index = 0;
                    return Command::perform(async {}, |_| Message::PlayNext);
                }
            }
            PlayNext => {
                let seq = self.seq.as_ref().unwrap();
                let event = seq.events[self.seq_index];

                self.seq_index += 1;
                let at_end = self.seq_index == seq.events.len();

                return Command::perform(
                    async move {
                        input::simulate(event).await;
                        if !at_end {
                            Message::PlayNext
                        } else {
                            Message::PlayStop
                        }
                    },
                    |m| m,
                );
            }
            PlayStop => self.playing = false,

            Loop(looping) => self.looping = looping,

            Save => {
                self.recording = false;
                self.playing = false;
                if let Some(seq) = &self.seq {
                    if let Some(path) = pick_file(true) {
                        data::save(seq, &path);
                        self.file = Some(path);
                        self.dirty = false;
                    }
                }
            }
            Load => {
                self.recording = false;
                self.playing = false;
                if let Some(path) = pick_file(false) {
                    self.seq = Some(data::load(&path));
                    self.file = Some(path);
                }
            }
        };
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        self.channel.subscription()
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
            .push(Text::new(format!("Current File: {:?}", self.file)))
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
