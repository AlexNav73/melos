
mod song;
mod sources;
mod controls;

pub use self::song::*;
pub use self::sources::Sample;

use rodio::Source;

use std::fmt;

pub trait DirectAccess: Source + fmt::Debug
    where Self::Item: ::rodio::Sample
{
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

pub trait Resettable: Source + fmt::Debug 
    where Self::Item: ::rodio::Sample
{
    fn reset(&mut self);
}

pub trait FloatWindow: Resettable
    where Self::Item: ::rodio::Sample
{
    fn play(&mut self, time: TimeSpan);
    fn end(&self) -> usize;
    fn cursor(&self) -> usize;

    fn current_sec(&self) -> usize {
        self.cursor() / self.channels() as usize / self.samples_rate() as usize
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

