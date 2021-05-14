#![windows_subsystem = "windows"]
#![feature(once_cell)]

mod data;
mod play;
mod record;
mod ui;

fn main() {
    record::init();

    dbg!(std::mem::size_of::<data::Event>());

    ui::show();

    println!("recording");
    record::start();
    std::thread::sleep(std::time::Duration::from_secs(10));
    let seq = record::stop();

    println!("playing");
    play::start(&seq);
}
