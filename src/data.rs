use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub pre_delay: Duration,
    pub ty: rdev::EventType,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    pub events: Vec<Event>,
}

pub fn save(seq: &Sequence, path: &Path) {
    let file = File::create(path).unwrap();
    let file = BufWriter::new(file);
    bincode::serialize_into(file, seq).unwrap();
}
pub fn load(path: &Path) -> Sequence {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    bincode::deserialize_from(file).unwrap()
}
