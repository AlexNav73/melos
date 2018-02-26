
use super::TimeSpan;

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Controls {
    pub stopped: AtomicBool,
    pub paused: AtomicBool,
    pub time: Mutex<TimeSpan>,
    pub volume: Mutex<f32>,
    pub progress: Mutex<u32>,
}

impl Controls {
    #[inline]
    pub fn new() -> Self {
        Controls {
            stopped: false.into(),
            paused: false.into(),
            time: Mutex::new(TimeSpan::default()),
            volume: Mutex::new(1.0),
            progress: Mutex::new(0),
        }
    }

    #[inline]
    pub fn stopped(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn set_stopped(&self, value: bool) {
        self.stopped.store(value, Ordering::SeqCst);
    }

    #[inline]
    pub fn paused(&self) -> bool {
        self.paused.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn set_paused(&self, value: bool) {
        self.paused.store(value, Ordering::SeqCst);
    }

    #[inline]
    pub fn volume(&self) -> f32 {
        *self.volume.lock().unwrap()
    }

    #[inline]
    pub fn set_volume(&self, value: f32) {
        *self.volume.lock().unwrap() = value;
    }

    #[inline]
    pub fn time(&self) -> TimeSpan {
        *self.time.lock().unwrap()
    }

    #[inline]
    pub fn set_time(&self, value: TimeSpan) {
        *self.time.lock().unwrap() = value;
    }

    #[inline]
    pub fn progress(&self) -> u32 {
        *self.progress.lock().unwrap()
    }

    #[inline]
    pub fn set_progress(&self, value: u32) {
        *self.progress.lock().unwrap() = value;
    }
}

