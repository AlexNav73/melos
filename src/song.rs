
use rodio;
use rodio::Source;
use rodio::buffer::SamplesBuffer;

use std::path::Path;
use std::io::BufReader;
use std::fs::File;
use std::sync::mpsc::{channel, Sender};

#[derive(Clone)]
pub struct Song {
    sender: Sender<SongMsg>
}

impl Song {
    pub fn new<P: ToString>(path: P) -> Self {
        use std::thread;

        let path = path.to_string();
        let (tx, rx) = channel();

        thread::spawn(move || {
            let mut song = SongThread::new(path);
            for msg in rx.iter() {
                match msg {
                    SongMsg::Open(p) => song = SongThread::new(p),
                    m => song.handle(m),
                }
            }
        });

        Song { sender: tx }
    }

    pub fn play(&self, time: (u32, u32)) {
        self.sender.send(SongMsg::Play(time)).unwrap()
    }

    pub fn stop(&self) {
        self.sender.send(SongMsg::Stop).unwrap()
    }

    pub fn pause(&self) {
        self.sender.send(SongMsg::Pause).unwrap()
    }

    pub fn volume(&self, volume: f32) {
        self.sender.send(SongMsg::Volume(volume)).unwrap();
    }
}

enum SongMsg {
    Open(String),
    Play((u32, u32)),
    Stop,
    Pause,
    Volume(f32)
}

type Sample = i16;

struct SongThread {
    samples: Vec<Sample>,
    sink: rodio::Sink,
    endpoint: rodio::Endpoint,
    samples_rate: u32,
    channels: u16
}

impl SongThread {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path.as_ref()).expect("Invalid file name");
        let endpoint = rodio::get_endpoints_list().next().unwrap();
        let sink = rodio::Sink::new(&endpoint);
        let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
        let samples_rate = decoder.samples_rate();
        let channels = decoder.channels();

        SongThread {
            samples: decoder.collect::<Vec<_>>(),
            channels,
            sink,
            endpoint,
            samples_rate,
        }
    }

    fn play(&self, (start, duration): (u32, u32)) {
        if !self.sink.is_paused() {
            let source = self.take_part(start, duration);
            self.sink.append(source);
        }
        self.sink.play();
    }


    fn stop(&mut self) {
        self.sink = rodio::Sink::new(&self.endpoint);
    }

    fn pause(&self) {
        self.sink.pause();
    }

    fn volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }

    fn handle(&mut self, msg: SongMsg) {
        match msg {
            SongMsg::Play(t) => self.play(t),
            SongMsg::Stop => self.stop(),
            SongMsg::Pause => self.pause(),
            SongMsg::Volume(v) => self.volume(v),
            SongMsg::Open(_) => {}
        }
    }

    fn take_part(&self, start: u32, duration: u32) -> SamplesBuffer<Sample> {
        let source = self.samples.iter()
            .skip(self.channels as usize * self.samples_rate as usize * start as usize)
            .take(self.channels as usize * self.samples_rate as usize * duration as usize)
            .cloned()
            .collect::<Vec<Sample>>();
        SamplesBuffer::new(self.channels, self.samples_rate, source)
    }
}

