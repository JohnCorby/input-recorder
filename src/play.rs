use crate::input::Sequence;

pub fn start(seq: &Sequence) {
    for event in &seq.events {
        std::thread::sleep(event.pre_delay);
        println!("p {:?}", event);
        rdev::simulate(&event.ty).unwrap();
    }
}
