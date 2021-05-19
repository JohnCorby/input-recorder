//! attempt at a global way to pass message into (and maybe out of?) the app
//! the `static mut`s assume there is ONLY 1 APP, hence no mutex

use crate::Message;
use iced::Subscription;
use iced_futures::futures::stream;
use iced_futures::BoxStream;
use parking_lot::Mutex;
use tokio::sync::{broadcast, mpsc};

fn check_concurrency() {
    debug_assert_eq!(
        std::thread::current().name(),
        Some("main"),
        "this aint the main thread. are you trying to use this concurrently?"
    );
}

type Static<T> = Mutex<Option<T>>;

pub static IN_TX: Static<mpsc::Sender<Message>> = Mutex::new(None);
pub static mut _IN_RX: Option<mpsc::Receiver<Message>> = None;

pub static OUT_RX: Static<broadcast::Receiver<Message>> = Mutex::new(None);
pub static mut _OUT_TX: Option<broadcast::Sender<Message>> = None;

pub fn init() {
    check_concurrency();

    const BUFFER: usize = 1000;
    let (tx, rx) = mpsc::channel(BUFFER);
    *IN_TX.lock() = Some(tx);
    unsafe { _IN_RX = Some(rx) };

    let (tx, rx) = broadcast::channel(BUFFER);
    *OUT_RX.lock() = Some(rx);
    unsafe { _OUT_TX = Some(tx) };
}

pub fn outgoing(message: Message) {
    check_concurrency();

    let tx = unsafe { _OUT_TX.as_mut().unwrap() };
    tx.send(message).unwrap();

    // println!("outgoing: {:?}", message);
}

pub fn incoming() -> Subscription<Message> {
    check_concurrency();

    struct Recipe;
    impl<H: std::hash::Hasher, E> iced_futures::subscription::Recipe<H, E> for Recipe {
        type Output = Message;

        fn hash(&self, state: &mut H) {
            use std::hash::Hash;
            std::any::TypeId::of::<Self>().hash(state);
        }

        fn stream(self: Box<Self>, _: BoxStream<E>) -> BoxStream<Message> {
            let stream = stream::unfold((), |_| async move {
                let rx = unsafe { _IN_RX.as_mut().unwrap() };
                let message = rx.recv().await.unwrap();

                // println!("incoming: {:?}", message);

                Some((message, ()))
            });

            Box::pin(stream)
        }
    }

    Subscription::from_recipe(Recipe)
}
