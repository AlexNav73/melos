
use rodio::{Sample as Sample_, Source};

use std::ops::{Deref, DerefMut};
use std::time::Duration;

pub struct PausableSource<T>
    where T: Source,
          <T as Iterator>::Item: Sample_
{
    source: T,
    paused: bool
}

impl<T> PausableSource<T>
    where T: Source,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    pub fn new(source: T) -> Self {
        PausableSource { source, paused: false }
    }

    #[inline]
    pub fn pause(&mut self, pause: bool) {
        self.paused = pause;
    }

}

impl<T> Source for PausableSource<T>
    where T: Source,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.source.channels()
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.source.total_duration()
    }
}

impl<T> Iterator for PausableSource<T>
    where T: Source,
          <T as Iterator>::Item: Sample_
{
    type Item = <T as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.paused {
            Some(<T as Iterator>::Item::zero_value())
        } else {
            self.source.next()
        }
    }
}

impl<T> Deref for PausableSource<T>
    where T: Source,
          <T as Iterator>::Item: Sample_
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<T> DerefMut for PausableSource<T>
    where T: Source,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source
    }
}
