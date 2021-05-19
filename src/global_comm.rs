//! attempt at a global way to pass message into (and maybe out of?) the app
//! the `static mut`s assume there is ONLY 1 APP, hence no mutex

use crate::Message;
use iced::Subscription;
use iced_futures::futures::stream;
use iced_futures::BoxStream;
use std::lazy::SyncOnceCell;
use tokio::sync::{broadcast, mpsc};

pub static IN_TX: SyncOnceCell<mpsc::Sender<Message>> = SyncOnceCell::new();
pub static _IN_RX: SyncOnceCell<mpsc::Receiver<Message>> = SyncOnceCell::new();

pub static OUT_RX: SyncOnceCell<broadcast::Receiver<Message>> = SyncOnceCell::new();
pub static _OUT_TX: SyncOnceCell<broadcast::Sender<Message>> = SyncOnceCell::new();

pub fn init() {
    const BUFFER: usize = 1000;
    let (tx, rx) = mpsc::channel(BUFFER);
    IN_TX.set(tx).unwrap();
    _IN_RX.set(rx).unwrap();

    let (tx, rx) = broadcast::channel(BUFFER);
    OUT_RX.set(rx).unwrap();
    _OUT_TX.set(tx).unwrap();
}

pub fn outgoing(message: Message) {
    let tx = _OUT_TX.get().unwrap();
    tx.send(message).unwrap();

    // println!("outgoing: {:?}", message);
}

pub fn incoming() -> Subscription<Message> {
    struct Recipe;
    impl<H: std::hash::Hasher, E> iced_futures::subscription::Recipe<H, E> for Recipe {
        type Output = Message;

        fn hash(&self, state: &mut H) {
            use std::hash::Hash;
            std::any::TypeId::of::<Self>().hash(state);
        }

        fn stream(self: Box<Self>, _: BoxStream<E>) -> BoxStream<Message> {
            let stream = stream::unfold((), |_| async move {
                let rx = _IN_RX.get().unwrap();
                let rx = unsafe {
                    // fuck you
                    #[allow(mutable_transmutes)]
                    std::mem::transmute::<&mpsc::Receiver<Message>, &mut mpsc::Receiver<Message>>(
                        rx,
                    )
                };
                let message = rx.recv().await.unwrap();

                // println!("incoming: {:?}", message);

                Some((message, ()))
            });

            Box::pin(stream)
        }
    }

    Subscription::from_recipe(Recipe)
}
