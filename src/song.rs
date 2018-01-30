
use rodio;
use rodio::Source;
use rodio::queue;
use rodio::buffer::SamplesBuffer;

use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::sync::mpsc::Receiver;

type Sample = i16;

pub struct Song {
    controls: Arc<Controls>,
    queue_tx: Arc<queue::SourcesQueueInput<f32>>,
    sleep_until_end: Arc<Mutex<Option<Receiver<()>>>>,
}

struct Controls {
    stopped: AtomicBool,
    paused: AtomicBool,
    time: Mutex<TimeSpan>,
    volume: Mutex<f32>,
}

#[derive(Copy, Clone)]
pub struct TimeSpan {
    start: u32,
    duration: u32
}

impl TimeSpan {
    pub fn new(start: u32, duration: u32) -> Self {
        TimeSpan { start, duration }
    }
}

impl Song {
    pub fn new() -> Self {
        let endpoint = rodio::get_endpoints_list().next().unwrap();
        let (queue_tx, queue_rx) = queue::queue(true);
        rodio::play_raw(&endpoint, queue_rx);

        let controls = Arc::new(Controls {
            stopped: false.into(),
            paused: false.into(),
            time: Mutex::new(TimeSpan { start: 0, duration: 0 }),
            volume: Mutex::new(1.0),
        });

        Song { controls, queue_tx, sleep_until_end: Arc::new(Mutex::new(None)) }
    }

    pub fn open<P: ToString>(&self, path: P) {
        use std::thread;

        let path = path.to_string();
        let controls = self.controls.clone();
        let queue = self.queue_tx.clone();
        let sleep_until_end = self.sleep_until_end.clone();

        thread::spawn(move || {
            let file = File::open(path).expect("Invalid file name");
            let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();

            let samples_rate = decoder.samples_rate();
            let channels = decoder.channels();
            let samples = decoder.collect::<Vec<_>>();
            let duration_ns = 1_000_000_000u64.checked_mul(samples.len() as u64).unwrap() /
                samples_rate as u64 / channels as u64;
            let duration = Duration::new(duration_ns / 1_000_000_000,
                                         (duration_ns % 1_000_000_000) as u32);

            let source = TestSource::new(channels, samples_rate, duration, samples)
                .amplify(1.0)
                .periodic_access(Duration::from_millis(5), move |src| {
                    src.inner_mut().stop(controls.stopped.load(Ordering::SeqCst));
                    src.inner_mut().pause(controls.paused.load(Ordering::SeqCst));
                    src.inner_mut().play(*controls.time.lock().unwrap());
                    src.set_factor(*controls.volume.lock().unwrap());
                })
                .convert_samples();

            *sleep_until_end.lock().unwrap() = Some(queue.append_with_signal(source));

            println!("Song has been loaded");
        });
    }

    #[inline]
    pub fn sleep_until_end(&self) {
        if let Some(sleep_until_end) = self.sleep_until_end.lock().unwrap().take() {
            let _ = sleep_until_end.recv();
        }
    }

    #[inline]
    pub fn play(&self, time: TimeSpan) {
        *self.controls.time.lock().unwrap() = time;
        self.controls.stopped.store(false, Ordering::SeqCst);
        self.controls.paused.store(false, Ordering::SeqCst);
    }

    #[inline]
    pub fn stop(&self) {
        self.controls.stopped.store(true, Ordering::SeqCst);
    }

    #[inline]
    pub fn pause(&self) {
        self.controls.paused.store(true, Ordering::SeqCst);
    }

    #[inline]
    pub fn volume(&mut self, volume: f32) {
        *self.controls.volume.lock().unwrap() = volume;
    }
}

struct TestSource {
    start: usize,
    end: usize,
    current: usize,
    stopped: bool,
    paused: bool,
    debug: bool,
    source: Vec<Sample>,
    channels: u16,
    samples_rate: u32,
    duration: Duration
}

impl TestSource {
    #[inline]
    fn new(channels: u16, samples_rate: u32, duration: Duration, source: Vec<Sample>) -> Self {
        TestSource {
            start: 0,
            end: 0,
            current: 0,
            stopped: false,
            paused: false,
            debug: false,
            source,
            channels,
            samples_rate,
            duration
        }
    }

    #[inline]
    fn stop(&mut self, stop: bool) {
        self.stopped = stop;
    }

    #[inline]
    fn pause(&mut self, pause: bool) {
        self.paused = pause;
    }

    #[inline]
    fn play(&mut self, time: TimeSpan) {
        let multiplayer = self.channels as usize * self.samples_rate as usize;
        self.start = multiplayer * time.start as usize;
        self.end = self.start + (multiplayer * time.duration as usize);
        if self.current < self.start || self.current > self.end {
            self.current = self.start;
        }
    }
}

impl fmt::Debug for TestSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TestSource")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("current", &self.current)
            .field("stopped", &self.stopped)
            .field("paused", &self.paused)
            .field("channels", &self.channels)
            .field("samples_rate", &self.samples_rate)
            .finish()
    }
}

impl Iterator for TestSource {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        use rodio::Sample as _Sample;

        if self.current % 15 == 0 {
            println!("{:?}", self);
        }

        if self.paused {
            return Some(Sample::zero_value());
        } else if self.stopped {
            self.current = self.start;
            return Some(Sample::zero_value());
        } else if self.current < self.end {
            let ret = self.source.get(self.current).cloned();
            self.current += 1;
            return ret;
        } else {
            return Some(Sample::zero_value());
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.source.iter().size_hint()
    }
}

impl Source for TestSource {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.channels
    }

    #[inline]
    fn samples_rate(&self) -> u32 {
        self.samples_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        Some(self.duration)
    }
}

impl Drop for Song {
    #[inline]
    fn drop(&mut self) {
        self.queue_tx.set_keep_alive_if_empty(false);
        self.controls.stopped.store(true, Ordering::Relaxed);
    }
}

