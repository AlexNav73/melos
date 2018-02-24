
mod song;
mod sources;
mod controls;

pub use self::song::*;

use rodio::{Sample, Source};

use std::fmt;

pub trait DirectAccess: Source + fmt::Debug
    where Self::Item: Sample
{
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

pub trait FloatWindow: Source + fmt::Debug 
    where Self::Item: Sample
{
    fn play(&mut self, time: TimeSpan);
    fn end(&self) -> usize;
    fn current(&self) -> usize;
    fn reset(&mut self);

    fn cursor(&self) -> usize {
        self.current() / self.channels() as usize / self.samples_rate() as usize
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

