use crate::input::*;
use druid::widget::{Button, Flex};
use druid::{AppLauncher, Size, Widget, WidgetExt, WindowDesc};
use std::sync::atomic::Ordering::Relaxed;

pub fn show() {
    let window = WindowDesc::new(build_ui)
        .title("Input Recorder")
        .window_size(Size::default());
    AppLauncher::with_window(window).launch(false).unwrap()
}

fn build_ui() -> impl Widget<bool> {
    Flex::column()
        .with_child(
            Button::dynamic(|_: &bool, _| {
                format!(
                    "{} recording",
                    if RECORDING.load(Relaxed) {
                        "stop"
                    } else {
                        "start"
                    }
                )
            })
            .on_click(|_, data: &mut bool, _| {
                if !RECORDING.load(Relaxed) {
                    record_start();
                } else {
                    record_stop();
                }
                *data = !*data;
            }),
        )
        .with_child(Button::new("play").on_click(|_, data: &mut bool, _| {
            if RECORDING.load(Relaxed) {
                record_stop();
                *data = !*data;
            }
            play();
        }))
        .with_default_spacer()
        // .with_child(play_button)
        // .with_child(play_button)
        .center()
}
