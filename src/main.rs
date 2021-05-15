#![windows_subsystem = "windows"]
#![feature(once_cell)]

mod data;
mod input;
mod ui;

fn main() {
    dbg!(std::mem::size_of::<data::Event>());
    input::record_init();
    ui::show();
}
