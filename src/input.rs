use crate::channel::Channel;
use crate::data::{Event, Sequence};
use crate::Message;
use parking_lot::Mutex;
use smol::channel::Sender;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::time::SystemTime;

/// handles recording and playing
#[derive(Debug)]
pub struct Input {
    tx: Arc<Sender<Message>>,

    recording: AtomicBool,
    playing: AtomicBool,
    looping: AtomicBool,

    seq: Mutex<Sequence>,

    play_task: Mutex<Option<smol::Task<()>>>,
}

impl Input {
    pub fn new(channel: &Channel) -> Arc<Self> {
        let this = Self {
            tx: channel.tx.clone(),

            recording: Default::default(),
            playing: Default::default(),
            looping: Default::default(),

            seq: Default::default(),

            play_task: Default::default(),
        };
        let this = Arc::new(this);
        this.init_listener();
        this
    }

    fn init_listener(self: &Arc<Self>) {
        let this = self.clone();
        std::thread::Builder::new()
            .name("keyboard listener".into())
            .spawn(|| {
                let mut prev_time = SystemTime::now();

                rdev::listen(move |event| {
                    // slightly less crappy panic button
                    if let rdev::EventType::KeyPress(rdev::Key::End) = event.event_type {
                        this.play_stop();
                    }

                    if !this.recording.load(Relaxed) {
                        return;
                    }

                    // fixme absolute mouse movement = games spazz out = defeats whole point of this project :|
                    let time = event.time;
                    // no delay at beginning
                    if this.seq.lock().events.is_empty() {
                        prev_time = time;
                    }
                    let event = Event {
                        pre_delay: time.duration_since(prev_time).unwrap(),
                        ty: event.event_type,
                    };
                    // println!("{:?}", event);
                    this.seq.lock().events.push(event);
                    prev_time = time;
                })
                .unwrap()
            })
            .unwrap();
    }

    pub fn rec_start(&self) {
        if self.recording.load(Relaxed) {
            return;
        }

        if self.playing.load(Relaxed) {
            self.play_stop()
        }

        println!("START RECORDING");
        self.seq.lock().events.clear();
        self.recording.store(true, Relaxed);
        self.tx.try_send(Message::Recording(true)).unwrap();

        self.tx.try_send(Message::Dirty(true)).unwrap();
    }
    pub fn rec_stop(&self) {
        if !self.recording.load(Relaxed) {
            return;
        }

        println!("STOP RECORDING");
        self.recording.store(false, Relaxed);
        self.tx.try_send(Message::Recording(false)).unwrap();
    }

    pub fn play_start(self: &Arc<Self>) {
        if self.playing.load(Relaxed) {
            return;
        }

        if self.seq.lock().events.is_empty() {
            return;
        }

        if self.recording.load(Relaxed) {
            self.rec_stop()
        }

        println!("START PLAYING");
        self.playing.store(true, Relaxed);
        self.tx.try_send(Message::Playing(true)).unwrap();

        let events = self.seq.lock().events.clone();
        let this = self.clone();
        let task = smol::spawn(async move {
            loop {
                for event in &events {
                    smol::Timer::after(event.pre_delay).await;
                    // println!("{:?}", event);
                    rdev::simulate(&event.ty).unwrap();
                }
                if !this.looping.load(Relaxed) {
                    break;
                }
            }
            this.play_stop();
        });
        *self.play_task.lock() = Some(task);
    }
    pub fn play_stop(&self) {
        if !self.playing.load(Relaxed) {
            return;
        }

        println!("STOP PLAYING");
        self.playing.store(false, Relaxed);
        self.tx.try_send(Message::Playing(false)).unwrap();

        // cancels the task
        *self.play_task.lock() = None
    }

    pub fn looping(&self, value: bool) {
        self.looping.store(value, Relaxed);
        self.tx.try_send(Message::Looping(value)).unwrap();
    }

    pub fn save(&self) {
        if self.recording.load(Relaxed) {
            self.rec_stop()
        }
        if self.playing.load(Relaxed) {
            self.play_stop()
        }

        if !self.seq.lock().events.is_empty() {
            if let Some(path) = Self::pick_file(true) {
                crate::data::save(&self.seq.lock(), &path);
                self.tx.try_send(Message::File(path)).unwrap();
                self.tx.try_send(Message::Dirty(false)).unwrap();
            }
        }
    }
    pub fn load(&self) {
        if self.recording.load(Relaxed) {
            self.rec_stop()
        }
        if self.playing.load(Relaxed) {
            self.play_stop()
        }

        if let Some(path) = Self::pick_file(false) {
            *self.seq.lock() = crate::data::load(&path);
            self.tx.try_send(Message::File(path)).unwrap();
        }
    }

    fn pick_file(save: bool) -> Option<PathBuf> {
        const FILE_EXT: &str = "irs";
        use nfd::DialogType::*;
        let res = nfd::open_dialog(
            Some(FILE_EXT),
            Some(std::env::current_dir().unwrap().to_str().unwrap()),
            if save { SaveFile } else { SingleFile },
        )
        .unwrap();
        use nfd::Response::*;
        match res {
            Okay(path) => Some(path.into()),
            OkayMultiple(_) => unreachable!(),
            Cancel => None,
        }
    }
}
