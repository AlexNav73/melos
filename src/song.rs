
//use hound;
use rodio;
use rodio::Source;
use rodio::buffer::SamplesBuffer;
//use hound::{WavReader, WavWriter, WavSpec};

use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter, Read, Cursor};
use std::fs::File;

pub struct Song {
    samples: Vec<i16>,
    sink: rodio::Sink,
    samples_rate: u32,
    channels: u16
}

impl Song {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).expect("Invalid file name");
        let endpoint = rodio::get_endpoints_list().next().unwrap();
        let sink = rodio::Sink::new(&endpoint);
        let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
        let samples_rate = decoder.samples_rate();
        let channels = decoder.channels();

        Song {
            samples: decoder.collect::<Vec<_>>(),
            channels,
            sink,
            samples_rate,
        }
    }

    // D:\Programms\Rust\melos\samples\sonne.wav

    pub fn play(&self, start: u32, duration: u32) {
        let source = self.samples.iter()
            .skip(self.channels as usize * self.samples_rate as usize * start as usize)
            .take(self.channels as usize * self.samples_rate as usize * duration as usize)
            .map(|x| *x)
            .collect::<Vec<i16>>();
        let source = SamplesBuffer::new(self.channels, self.samples_rate, source);
        self.sink.append(source);
        self.sink.play();
    }

    pub fn stop(&self) {
        unimplemented!()
    }
}

