use crate::data::{Event, Sequence};
use crate::global_comm::IN_TX;
use crate::{App, Message};
use std::time::SystemTime;

pub fn init() {
    std::thread::Builder::new()
        .name("keyboard listener".into())
        .spawn(|| {
            // safe because it's used only in this fn and only on this thread
            static mut PREV_TIME: Option<SystemTime> = None;
            unsafe { PREV_TIME = Some(SystemTime::now()) }

            rdev::listen(move |event| {
                // slightly less crappy panic button
                if let rdev::EventType::KeyPress(rdev::Key::End) = event.event_type {
                    IN_TX
                        .lock()
                        .as_ref()
                        .unwrap()
                        .try_send(Message::PlayStop)
                        .unwrap();
                }

                // fixme absolute mouse movement = games spazz out = defeats whole point of this project :|
                let time = event.time;
                let event = Event {
                    pre_delay: time.duration_since(unsafe { PREV_TIME.unwrap() }).unwrap(),
                    ty: event.event_type,
                };
                // println!("{:?}", event);
                IN_TX
                    .lock()
                    .as_ref()
                    .unwrap()
                    .try_send(Message::InputEvent(event))
                    .unwrap();
                unsafe { PREV_TIME = Some(time) }
            })
            .unwrap()
        })
        .unwrap();
}

impl App {}
// pub fn record_start() {
//     println!("START RECORDING");
//     CURRENT_SEQ.lock().events.clear();
//     *PREV_TIME.lock() = Some(SystemTime::now());
//     RECORDING.store(true, Relaxed)
// }
// pub fn record_stop() {
//     println!("STOP RECORDING");
//     RECORDING.store(false, Relaxed);
// }

pub async fn play(seq: Sequence, looping: bool) {
    println!("START PLAYING");
    loop {
        for event in &seq.events {
            tokio::time::sleep(event.pre_delay).await;
            // println!("{:?}", event);
            rdev::simulate(&event.ty).unwrap();
        }
        if !looping {
            break;
        }
    }
    println!("STOP PLAYING");
}
