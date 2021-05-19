use crate::data::{Event, Sequence};
use crate::Message;
use smol::channel::Sender;
use std::time::SystemTime;

pub fn init_listener(tx: Sender<Message>) {
    std::thread::Builder::new()
        .name("keyboard listener".into())
        .spawn(|| {
            let mut prev_time = SystemTime::now();

            rdev::listen(move |event| {
                // slightly less crappy panic button
                if let rdev::EventType::KeyPress(rdev::Key::End) = event.event_type {
                    tx.try_send(Message::PlayStop).unwrap();
                }

                // fixme absolute mouse movement = games spazz out = defeats whole point of this project :|
                let time = event.time;
                let event = Event {
                    pre_delay: time.duration_since(prev_time).unwrap(),
                    ty: event.event_type,
                };
                // println!("{:?}", event);
                tx.try_send(Message::InputEvent(event)).unwrap();
                prev_time = time;
            })
            .unwrap()
        })
        .unwrap();
}

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
            smol::Timer::after(event.pre_delay).await;
            // println!("{:?}", event);
            rdev::simulate(&event.ty).unwrap();
        }
        if !looping {
            break;
        }
    }
    println!("STOP PLAYING");
}
