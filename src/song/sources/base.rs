
use super::DirectAccess;
use super::Sample;

use rodio::Source;

use std::fmt;
use std::time::Duration;
use std::sync::Arc;

pub struct BaseSource {
    channels: u16,
    samples_rate: u32,
    duration: Duration,
    source: Arc<Vec<Sample>>
}

impl BaseSource {
    pub fn new(channels: u16, samples_rate: u32, source: Arc<Vec<Sample>>) -> Self {
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
}

impl Iterator for BaseSource {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        use rodio::Sample as _Sample;

        return Some(Sample::zero_value());
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
    fn get(&self, index: usize) -> Option<&Sample> {
        self.source.get(index)
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

