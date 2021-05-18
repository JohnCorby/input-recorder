#![windows_subsystem = "windows"]

mod data;
mod global_comm;
mod input;
mod ui;

fn main() {
    // dbg!(std::mem::size_of::<data::Event>());
    ui::show();
}
