
use super::DirectAccess;
use super::Sample;

use rodio::Source;

use std::fmt;
use std::time::Duration;

// TODO(alex): Rename to something more appropriate
pub struct BaseSource {
    current: usize,
    channels: u16,
    samples_rate: u32,
    duration: Duration,
    source: Vec<Sample> // TODO(alex): abstract over samples type
}

impl BaseSource {
    pub fn new(channels: u16, samples_rate: u32, source: Vec<Sample>) -> Self {
        let duration_ns = 1_000_000_000u64.checked_mul(source.len() as u64).unwrap() /
            samples_rate as u64 / channels as u64;
        let duration = Duration::new(duration_ns / 1_000_000_000,
                                    (duration_ns % 1_000_000_000) as u32);

        BaseSource {
            current: 0,
            channels,
            samples_rate,
            duration,
            source,
        }
    }
}

impl Iterator for BaseSource {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        self.source.get(self.current - 1).cloned()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.source.iter().size_hint()
    }
}

impl Source for BaseSource {
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

impl DirectAccess for BaseSource {
    #[inline]
    fn current(&self) -> usize {
        self.current
    }
    #[inline]
    fn set_current(&mut self, index: usize) {
        self.current = index;
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

