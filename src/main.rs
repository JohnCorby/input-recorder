#![windows_subsystem = "windows"]

mod data;
mod input;
mod ui;

fn main() {
    // dbg!(std::mem::size_of::<data::Event>());
    input::init();
    ui::show();
}
