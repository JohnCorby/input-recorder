use crate::data::{Event, Sequence};
use parking_lot::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::SystemTime;

static CURRENT_SEQ: Mutex<Sequence> = Mutex::new(Sequence { events: vec![] });

static RECORDING: AtomicBool = AtomicBool::new(false);
static PREV_TIME: Mutex<Option<SystemTime>> = Mutex::new(None);

pub fn init() {
    std::thread::Builder::new()
        .name("input listener".into())
        .spawn(|| {
            rdev::listen(|event| {
                // crappy panic button
                if let rdev::EventType::KeyPress(rdev::Key::End) = event.event_type {
                    std::process::exit(0)
                }

                if !RECORDING.load(Relaxed) {
                    return;
                }

                // fixme absolute mouse movement = games spazz out = defeats whole point of this project :|
                let time = event.time;
                let event = Event {
                    pre_delay: time.duration_since(PREV_TIME.lock().unwrap()).unwrap(),
                    ty: event.event_type,
                };
                // println!("{:?}", event);
                CURRENT_SEQ.lock().events.push(event);
                *PREV_TIME.lock() = Some(time)
            })
            .unwrap()
        })
        .unwrap();
}

pub fn record_start() {
    println!("START RECORDING");
    CURRENT_SEQ.lock().events.clear();
    *PREV_TIME.lock() = Some(SystemTime::now());
    RECORDING.store(true, Relaxed)
}
pub fn record_stop() {
    println!("STOP RECORDING");
    RECORDING.store(false, Relaxed);
}

/// note: blocks
pub fn play(looping: bool) {
    println!("START PLAYING");
    // fixme this is the worst ever, but fixing it is super super painful
    if looping {
        loop {
            for event in &CURRENT_SEQ.lock().events {
                std::thread::sleep(event.pre_delay);
                // println!("{:?}", event);
                rdev::simulate(&event.ty).unwrap();
            }
        }
    } else {
        for event in &CURRENT_SEQ.lock().events {
            std::thread::sleep(event.pre_delay);
            // println!("{:?}", event);
            rdev::simulate(&event.ty).unwrap();
        }
    }
    println!("STOP PLAYING");
}
