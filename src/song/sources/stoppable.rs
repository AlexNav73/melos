
use super::Resettable;

use rodio::{Sample as Sample_, Source};

use std::ops::{Deref, DerefMut};
use std::time::Duration;

pub struct StoppableSource<T>
    where T: Resettable,
          <T as Iterator>::Item: Sample_
{
    source: T,
    stopped: bool
}

impl<T> StoppableSource<T>
    where T: Resettable,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    pub fn new(source: T) -> Self {
        StoppableSource { source, stopped: false }
    }

    #[inline]
    pub fn stop(&mut self, stop: bool) {
        self.stopped = stop;
    }

}

impl<T> Source for StoppableSource<T>
    where T: Resettable,
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

impl<T> Iterator for StoppableSource<T>
    where T: Resettable,
          <T as Iterator>::Item: Sample_
{
    type Item = <T as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            self.source.reset();
            Some(<T as Iterator>::Item::zero_value())
        } else {
            self.source.next()
        }
    }
}

impl<T> Deref for StoppableSource<T>
    where T: Resettable,
          <T as Iterator>::Item: Sample_
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<T> DerefMut for StoppableSource<T>
    where T: Resettable,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source
    }
}
