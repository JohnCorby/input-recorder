use crate::Message;
use iced::Subscription;
use iced_futures::futures::{stream, StreamExt};
use iced_futures::subscription::Recipe;
use iced_futures::BoxStream;
use smol::channel::{Receiver, Sender};
use std::sync::Arc;

/// lets you push messages into the app
#[derive(Debug)]
pub struct Channel {
    pub tx: Arc<Sender<Message>>,
    rx: Arc<Receiver<Message>>,
}

impl Channel {
    pub fn new() -> Self {
        let (tx, rx) = smol::channel::unbounded();
        Self {
            tx: tx.into(),
            rx: rx.into(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        struct Rx {
            rx: Arc<Receiver<Message>>,
        }
        impl<H: std::hash::Hasher, E> Recipe<H, E> for Rx {
            type Output = Message;

            fn hash(&self, state: &mut H) {
                use std::hash::Hash;
                std::any::TypeId::of::<Self>().hash(state);
            }

            fn stream(self: Box<Self>, _: BoxStream<E>) -> BoxStream<Message> {
                stream::unfold((), move |_| {
                    let rx = self.rx.clone();
                    async move {
                        let message = rx.recv().await.unwrap();
                        // println!("c {:?}", message);
                        Some((message, ()))
                    }
                })
                .boxed()
            }
        }
        Subscription::from_recipe(Rx {
            rx: self.rx.clone(),
        })
    }
}
