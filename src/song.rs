
use rodio;
use rodio::Source;
use rodio::buffer::SamplesBuffer;

use player::Player;

use std::path::Path;
use std::io::BufReader;
use std::fs::File;

pub struct Song {
    samples: Vec<i16>,
    sink: rodio::Sink,
    endpoint: rodio::Endpoint,
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
            endpoint,
            samples_rate,
        }
    }

    // D:\Programms\Rust\melos\samples\sonne.wav

    pub fn play(&self, player: &Player) {
        if !self.sink.is_paused() {
            let source = self.samples.iter()
                .skip(self.channels as usize * self.samples_rate as usize * player.start() as usize)
                .take(self.channels as usize * self.samples_rate as usize * player.duration() as usize)
                .cloned()
                .collect::<Vec<i16>>();
            let source = SamplesBuffer::new(self.channels, self.samples_rate, source);
            self.sink.append(source);
        }
        self.sink.play();
    }

    pub fn stop(&mut self) {
        self.sink = rodio::Sink::new(&self.endpoint);
    }
}

