//! how we show shit to the user

use druid::widget::{Button, Flex};
use druid::{AppLauncher, Widget, WindowDesc};

#[allow(dead_code)]
pub fn show() {
    let window = WindowDesc::new(build_ui);
    AppLauncher::with_window(window).launch(()).unwrap()
}

fn build_ui() -> impl Widget<()> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Button::new("start recording"))
                .with_child(Button::new("stop recording")),
        )
        .with_child(
            Flex::row()
                .with_child(Button::new("start playback"))
                .with_child(Button::new("stop playback")),
        )
}
