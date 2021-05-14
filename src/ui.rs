use druid::widget::{Button, Flex};
use druid::{AppLauncher, Widget, WindowDesc};

// todo use this

pub fn show() {
    let window = WindowDesc::new(build_ui);
    AppLauncher::with_window(window).launch(()).unwrap()
}

fn build_ui() -> impl Widget<()> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Button::new("start recording"))
                .with_child(Button::new("stop recording"))
                .with_child(Button::new("pause recording"))
                .with_child(Button::new("unpause recording")),
        )
        .with_child(
            Flex::row()
                .with_child(Button::new("start playback"))
                .with_child(Button::new("stop playback"))
                .with_child(Button::new("pause playback"))
                .with_child(Button::new("unpause playback")),
        )
}
