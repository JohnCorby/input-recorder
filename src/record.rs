use crate::input::{Event, Sequence};
use std::time::SystemTime;

pub fn start() -> Sequence {
    let mut prev_time = SystemTime::now();
    let mut seq = Sequence { events: vec![] };

    rdev::listen(move |event| {
        let time = event.time;
        let event = Event {
            pre_delay: time.duration_since(prev_time).unwrap(),
            ty: event.event_type,
        };
        println!("r {:?}", event);
        seq.events.push(event);
        prev_time = time
    })
    .unwrap();

    todo!("return sequence")
}
