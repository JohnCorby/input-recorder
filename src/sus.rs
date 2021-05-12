#![allow(unused)]

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{SampleFormat, SampleRate, StreamConfig};
use rodio::{Decoder, Source};
use std::io::Cursor;

pub fn sus() {
    let sus = include_bytes!("sus.mp3");

    let reader = Cursor::new(sus);
    let mut source = Decoder::new_mp3(reader).unwrap().convert_samples();

    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();

    let config = device
        .supported_output_configs()
        .unwrap()
        .next()
        .unwrap()
        .with_sample_rate(SampleRate(source.sample_rate()));
    let channels = config.channels() as usize;

    let stream = device.build_output_stream(
        &config.config(),
        move |output: &mut [f32], _| {
            for frame in output.chunks_mut(channels) {
                for sample in frame {
                    let value = source.next().unwrap();
                    *sample = value;
                }
            }
        },
        move |err| unreachable!(),
    );

    // std::thread::sleep(std::time::Duration::from_secs(5));
}
