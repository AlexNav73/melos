
mod song;
mod source;
mod controls;

pub use self::song::*;

use rodio::{Sample, Source};

use std::fmt;

pub trait DirectAccess: Source + fmt::Debug
    where Self::Item: Sample
{
    fn get(&self, index: usize) -> Option<&Self::Item>;
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

