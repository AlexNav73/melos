
use super::TimeSpan;
use super::controls::Controls;

use rodio::Source;

use std::fmt;
use std::time::Duration;
use std::sync::Arc;

type Sample = i16;

pub struct SmartSource {
    start: usize,
    end: usize,
    current: usize,
    stopped: bool,
    paused: bool,
    source: BaseSource,
    controls: Arc<Controls>
}

impl SmartSource {
    #[inline]
    pub fn new(source: BaseSource, controls: Arc<Controls>) -> Self {
        SmartSource {
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
    pub fn cursor(&self) -> usize {
        self.current / self.source.samples_rate_for_all_channels()
    }

    #[inline]
    pub fn stop(&mut self, stop: bool) {
        self.stopped = stop;
    }

    #[inline]
    pub fn pause(&mut self, pause: bool) {
        self.paused = pause;
    }

    #[inline]
    pub fn play(&mut self, time: TimeSpan) {
        let multiplayer = self.source.samples_rate_for_all_channels();
        self.start = multiplayer * time.start as usize;
        self.end = self.start + (multiplayer * time.duration as usize);
        if self.current < self.start || self.current > self.end {
            self.current = self.start;
        }
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
            self.source.get(self.current - 1)
                .cloned()
                .or(Some(Sample::zero_value()))
        } else {
            self.controls.set_stopped(true);
            return Some(Sample::zero_value());
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.source.size_hint()
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
            .field("source", &self.source)
            .finish()
    }
}

impl Source for SmartSource {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.source.channels()
    }

    #[inline]
    fn samples_rate(&self) -> u32 {
        self.source.samples_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.source.total_duration()
    }
}

pub struct BaseSource {
    channels: u16,
    samples_rate: u32,
    duration: Duration,
    source: Vec<Sample>
}

impl BaseSource {
    pub fn new(channels: u16, samples_rate: u32, source: Vec<Sample>) -> Self {
        let duration_ns = 1_000_000_000u64.checked_mul(source.len() as u64).unwrap() /
            samples_rate as u64 / channels as u64;
        let duration = Duration::new(duration_ns / 1_000_000_000,
                                    (duration_ns % 1_000_000_000) as u32);

        BaseSource {
            channels,
            samples_rate,
            duration,
            source,
        }
    }

    fn samples_rate_for_all_channels(&self) -> usize {
        self.channels as usize * self.samples_rate as usize
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&Sample> {
        self.source.get(index)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.source.iter().size_hint()
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

impl fmt::Debug for BaseSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BaseSource")
            .field("channels", &self.channels)
            .field("samples_rate", &self.samples_rate)
            .field("samples", &" ... ")
            .finish()
    }
}

