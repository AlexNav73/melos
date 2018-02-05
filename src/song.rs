
use rodio;
use rodio::Source;

use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

type Sample = i16;

pub struct Song {
    controls: Arc<Controls>,
}

struct Controls {
    stopped: AtomicBool,
    paused: AtomicBool,
    time: Mutex<TimeSpan>,
    volume: Mutex<f32>,
    progress: Mutex<u32>,
    loaded: AtomicBool
}

impl Controls {
    #[inline]
    fn new() -> Self {
        Controls {
            stopped: false.into(),
            paused: false.into(),
            time: Mutex::new(TimeSpan::default()),
            volume: Mutex::new(1.0),
            progress: Mutex::new(0),
            loaded: false.into()
        }
    }

    #[inline]
    fn stopped(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }

    #[inline]
    fn set_stopped(&self, value: bool) {
        self.stopped.store(value, Ordering::SeqCst);
    }

    #[inline]
    fn paused(&self) -> bool {
        self.paused.load(Ordering::SeqCst)
    }

    #[inline]
    fn set_paused(&self, value: bool) {
        self.paused.store(value, Ordering::SeqCst);
    }

    #[inline]
    fn volume(&self) -> f32 {
        *self.volume.lock().unwrap()
    }

    #[inline]
    fn set_volume(&self, value: f32) {
        *self.volume.lock().unwrap() = value;
    }

    #[inline]
    fn time(&self) -> TimeSpan {
        *self.time.lock().unwrap()
    }

    #[inline]
    fn set_time(&self, value: TimeSpan) {
        *self.time.lock().unwrap() = value;
    }

    #[inline]
    fn progress(&self) -> u32 {
        *self.progress.lock().unwrap()
    }

    #[inline]
    fn set_progress(&self, value: u32) {
        *self.progress.lock().unwrap() = value;
    }

    #[inline]
    fn loaded(&self) -> bool {
        self.loaded.swap(false, Ordering::SeqCst)
    }
}

#[derive(Copy, Clone, Default)]
pub struct TimeSpan {
    start: u32,
    duration: u32
}

impl TimeSpan {
    #[inline]
    pub fn new(start: u32, duration: u32) -> Self {
        TimeSpan { start, duration }
    }
}

impl Song {
    pub fn new() -> Self {
        Song { controls: Arc::new(Controls::new()) }
    }

    #[allow(deprecated)]
    pub fn open<P: ToString>(&self, path: P) {
        use std::thread;

        let path = path.to_string();
        let controls = self.controls.clone();

        thread::spawn(move || {
            let file = File::open(path).expect("Invalid file name");
            let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();

            let samples_rate = decoder.samples_rate();
            let channels = decoder.channels();
            let samples = decoder.collect::<Vec<_>>();
            let controls2 = controls.clone();
            let controls3 = controls.clone();

            let source = SmartSource::new(channels, samples_rate, samples, controls.clone())
                .amplify(1.0)
                .periodic_access(Duration::from_millis(5), move |src| {
                    src.inner_mut().stop(controls.stopped());
                    src.inner_mut().pause(controls.paused());
                    src.inner_mut().play(controls.time());
                    src.set_factor(controls.volume());
                })
                .periodic_access(Duration::from_millis(995), move |src| {
                    controls2.set_progress(src.inner().inner().cursor() as u32);
                })
                .convert_samples();

            let endpoint = rodio::get_endpoints_list().next().unwrap();
            rodio::play_raw(&endpoint, source);

            controls3.loaded.store(true, Ordering::SeqCst);
        });
    }

    #[inline]
    pub fn play(&self, time: TimeSpan) {
        self.controls.set_time(time);
        self.controls.set_stopped(false);
        self.controls.set_paused(false);
    }

    #[inline]
    pub fn stop(&self) {
        self.controls.set_stopped(true);
    }

    #[inline]
    pub fn pause(&self) {
        self.controls.set_paused(true);
    }

    #[inline]
    pub fn volume(&mut self, value: f32) {
        self.controls.set_volume(value);
    }

    #[inline]
    pub fn progress(&self) -> u32 {
        self.controls.progress()
    }

    #[inline]
    pub fn loaded(&self) -> bool {
        self.controls.loaded()
    }
}

struct SmartSource {
    start: usize,
    end: usize,
    current: usize,
    stopped: bool,
    paused: bool,
    source: Vec<Sample>,
    channels: u16,
    samples_rate: u32,
    duration: Duration,
    controls: Arc<Controls>
}

impl SmartSource {
    #[inline]
    fn new(channels: u16, samples_rate: u32, source: Vec<Sample>, controls: Arc<Controls>) -> Self {
        let duration_ns = 1_000_000_000u64.checked_mul(source.len() as u64).unwrap() /
            samples_rate as u64 / channels as u64;
        let duration = Duration::new(duration_ns / 1_000_000_000,
                                    (duration_ns % 1_000_000_000) as u32);

        SmartSource {
            channels,
            samples_rate,
            duration,
            source,
            controls,
            start: 0,
            end: 0,
            current: 0,
            stopped: false,
            paused: false,
        }
    }

    #[inline]
    fn cursor(&self) -> usize {
        self.current / self.channels as usize / self.samples_rate as usize
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

impl fmt::Debug for SmartSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SmartSource")
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

impl Iterator for SmartSource {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        use rodio::Sample as _Sample;

        if self.paused {
            return Some(Sample::zero_value());
        } else if self.stopped {
            self.current = self.start;
            return Some(Sample::zero_value());
        } else if self.current < self.end {
            self.current += 1;
            self.source.get(self.current - 1).cloned()
        } else {
            self.controls.set_stopped(true);
            return Some(Sample::zero_value());
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.source.iter().size_hint()
    }
}

impl Source for SmartSource {
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
        self.controls.stopped.store(true, Ordering::Relaxed);
    }
}

