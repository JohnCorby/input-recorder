use crate::data::{Event, Sequence};
use iced::Subscription;
use iced_futures::futures::stream;
use iced_futures::BoxStream;
use std::time::SystemTime;
use tokio::sync::mpsc::{channel, Receiver};

// unsafe, but is only used in 1 place and so should be okay
static mut MESSAGE_RX: Option<Receiver<Event>> = None;

pub fn init() {
    let (tx, rx) = channel(100);
    unsafe { MESSAGE_RX = Some(rx) }

    // awful but should technically be safe
    // since its only in this fn and only on this thread
    static mut PREV_TIME: SystemTime =
        unsafe { std::mem::transmute([0u8; std::mem::size_of::<SystemTime>()]) };
    unsafe { PREV_TIME = SystemTime::now() }

    std::thread::Builder::new()
        .name("keyboard listener".into())
        .spawn(|| {
            rdev::listen(move |event| {
                // crappy panic button
                if let rdev::EventType::KeyPress(rdev::Key::End) = event.event_type {
                    std::process::exit(0)
                }

                // fixme absolute mouse movement = games spazz out = defeats whole point of this project :|
                let time = event.time;
                let event = Event {
                    pre_delay: time.duration_since(unsafe { PREV_TIME }).unwrap(),
                    ty: event.event_type,
                };
                // println!("{:?}", event);
                tx.try_send(event).unwrap();
                unsafe { PREV_TIME = time }
            })
            .unwrap()
        })
        .unwrap();
}

pub fn subscription() -> Subscription<Event> {
    struct Recipe;
    impl<H: std::hash::Hasher, E> iced_futures::subscription::Recipe<H, E> for Recipe {
        type Output = Event;

        fn hash(&self, state: &mut H) {
            use std::hash::Hash;
            std::any::TypeId::of::<Self>().hash(state);
        }

        fn stream(self: Box<Self>, _: BoxStream<E>) -> BoxStream<Event> {
            let stream = stream::unfold((), |_| async move {
                let rx = unsafe { MESSAGE_RX.as_mut().unwrap() };
                let message = rx.recv().await.unwrap();
                Some((message, ()))
            });
            Box::pin(stream)
        }
    }

    Subscription::from_recipe(Recipe)
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
