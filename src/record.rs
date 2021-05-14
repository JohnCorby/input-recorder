use crate::data::{Event, Sequence};
use parking_lot::Mutex;
use std::time::SystemTime;

static RUNNING: Mutex<bool> = Mutex::new(false);
static SEQ: Mutex<Sequence> = Mutex::new(Sequence { events: vec![] });
static PREV_TIME: Mutex<Option<SystemTime>> = Mutex::new(None);

pub fn init() {
    *PREV_TIME.lock() = Some(SystemTime::now());

    std::thread::spawn(|| {
        rdev::listen(|event| {
            if !*RUNNING.lock() {
                return;
            }

            let time = event.time;
            let event = Event {
                pre_delay: time.duration_since(PREV_TIME.lock().unwrap()).unwrap(),
                ty: event.event_type,
            };
            println!("r {:?}", event);
            SEQ.lock().events.push(event);
            *PREV_TIME.lock() = Some(time)
        })
        .unwrap()
    });
}

// todo pause?
pub fn start() {
    *RUNNING.lock() = true
}
pub fn stop() -> Sequence {
    *RUNNING.lock() = false;
    std::mem::take(&mut *SEQ.lock())
}
