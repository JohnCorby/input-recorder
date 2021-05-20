#![feature(once_cell)]
#![windows_subsystem = "windows"]

mod channel;
mod data;
mod input;

use crate::channel::Channel;
use crate::input::Input;
use iced::{
    button, Align, Application, Button, Checkbox, Clipboard, Column, Command, Container, Element,
    Length, Settings, Space, Subscription, Text,
};
use std::path::PathBuf;
use std::sync::Arc;

fn main() {
    // dbg!(std::mem::size_of::<data::Event>());
    show();
}

pub fn show() {
    App::run(Settings::default()).unwrap()
}

#[derive(Debug)]
struct App {
    channel: Channel,
    input: Arc<Input>,

    recording: bool,
    playing: bool,
    looping: bool,
    dirty: bool,
    file: Option<PathBuf>,

    rec_button: button::State,
    play_button: button::State,
    save_button: button::State,
    load_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Recording(bool),
    Playing(bool),
    Looping(bool),
    Dirty(bool),
    File(PathBuf),

    RecButton,
    PlayButton,
    LoopCheckbox,
    SaveButton,
    LoadButton,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        let channel = Channel::new();
        let input = Input::new(&channel);

        (
            Self {
                channel,
                input,

                recording: false,
                playing: false,
                looping: false,
                dirty: false,
                file: None,

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
            self.file
        )
    }

    fn update(&mut self, message: Message, _: &mut Clipboard) -> Command<Message> {
        // println!("m {:?}", message);
        use Message::*;
        match message {
            Recording(value) => self.recording = value,
            Playing(value) => self.playing = value,
            Looping(value) => self.looping = value,
            Dirty(value) => self.dirty = value,
            File(value) => self.file = Some(value),

            RecButton => match !self.recording {
                true => self.input.rec_start(),
                false => self.input.rec_stop(),
            },
            PlayButton => match !self.playing {
                true => self.input.play_start(),
                false => self.input.play_stop(),
            },
            LoopCheckbox => self.input.looping(!self.looping),
            SaveButton => self.input.save(),
            LoadButton => self.input.load(),
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
        .on_press(Message::RecButton);
        let play = Button::new(
            &mut self.play_button,
            Text::new(format!(
                "{} Playing",
                if !self.playing { "Start" } else { "Stop" }
            )),
        )
        .on_press(Message::PlayButton);

        let content = Column::new()
            .align_items(Align::Center)
            .push(Text::new("Input Recorder of Piss"))
            .push(Space::with_height(Length::Fill))
            .push(rec)
            .push(play)
            .push(Checkbox::new(self.looping, "Loop", |_| {
                Message::LoopCheckbox
            }))
            .push(Space::with_height(Length::Fill))
            .push(Text::new(format!("Current File: {:?}", self.file)))
            .push(
                Button::new(&mut self.save_button, Text::new("Save")).on_press(Message::SaveButton),
            )
            .push(
                Button::new(&mut self.load_button, Text::new("Load")).on_press(Message::LoadButton),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
