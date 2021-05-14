#![windows_subsystem = "windows"]

mod input;
mod play;
mod record;
mod ui;

fn main() {
    // ui::show();
    let seq = record::start();
    play::start(&seq);
}
