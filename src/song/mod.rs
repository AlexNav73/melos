
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
    fn current(&self) -> usize;
    fn set_current(&mut self, index: usize);
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
}

pub trait Inspectable
    where Self: FloatWindow,
          <Self as Iterator>::Item: ::rodio::Sample
{
    fn current_sec(&self) -> usize;
}

impl<T> Inspectable for T
    where T: FloatWindow,
          <T as Iterator>::Item: ::rodio::Sample
{
    fn current_sec(&self) -> usize {
        self.cursor() / self.channels() as usize / self.sample_rate() as usize
    }
}

#[derive(Copy, Clone, Default)]
pub struct TimeSpan {
    pub start: u32,
    pub duration: u32
}

impl TimeSpan {
    #[inline]
    pub fn new(start: u32, duration: u32) -> Self {
        TimeSpan { start, duration }
    }
}

