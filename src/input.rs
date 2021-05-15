use crate::data::{Event, Sequence};
use parking_lot::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::SystemTime;

pub static RECORDING: AtomicBool = AtomicBool::new(false);
static CURRENT_SEQ: Mutex<Sequence> = Mutex::new(Sequence { events: vec![] });
static PREV_TIME: Mutex<Option<SystemTime>> = Mutex::new(None);

pub fn record_init() {
    std::thread::Builder::new()
        .name("input listener".into())
        .spawn(|| {
            rdev::listen(|event| {
                if !RECORDING.load(Relaxed) {
                    return;
                }

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
pub fn play() {
    println!("PLAY");

    for event in &CURRENT_SEQ.lock().events {
        std::thread::sleep(event.pre_delay);
        // println!("{:?}", event);
        rdev::simulate(&event.ty).unwrap();
    }
}
