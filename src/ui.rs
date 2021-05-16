use crate::input;
use druid::widget::{Button, Checkbox, Flex, Label};
use druid::{AppLauncher, Size, Widget, WidgetExt, WindowDesc};

#[derive(Debug, Default, Clone, druid::Data, druid::Lens)]
struct State {
    recording: bool,
    looping: bool,
}

pub fn show() {
    let window = WindowDesc::new(build_ui)
        .title("Input Recorder")
        .window_size(Size::default());
    AppLauncher::with_window(window)
        .launch(State::default())
        .unwrap()
}

fn build_ui() -> impl Widget<State> {
    Flex::column()
        .with_child(Label::new("Input Recorder of Piss"))
        .with_default_spacer()
        .with_child(
            Button::dynamic(|state: &State, _| {
                format!(
                    "{} Recording",
                    if state.recording { "Stop" } else { "Start" }
                )
            })
            .on_click(|_, state: &mut State, _| {
                if !state.recording {
                    state.recording = true;
                    input::record_start();
                } else {
                    state.recording = false;
                    input::record_stop();
                }
            }),
        )
        .with_child(Button::new("Play").on_click(|_, state: &mut State, _| {
            if state.recording {
                state.recording = false;
                input::record_stop();
            }
            input::play(state.looping);
        }))
        .with_child(Checkbox::new("Loop").lens(State::looping))
        .with_default_spacer()
        .with_child(Label::new("Current File: {}"))
        .with_child(Button::new("Save").on_click(|_, state: &mut State, _| {
            if state.recording {
                state.recording = false;
                input::record_stop();
            }
            // todo open file dialog
        }))
        .with_child(Button::new("Load").on_click(|_, state: &mut State, _| {
            if state.recording {
                state.recording = false;
                input::record_stop();
            }
            // todo save file dialog
        }))
        .center()
}
