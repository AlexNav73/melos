
use super::{TimeSpan, DirectAccess, FloatWindow, Resettable};

use rodio::{Source, Sample as Sample_};

use std::fmt;
use std::time::Duration;
use std::ops::Deref;

pub struct FloatWindowSource<T>
    where T: DirectAccess,
          <T as Iterator>::Item: Sample_
{
    start: usize,
    end: usize,
    source: T
}

impl<T> FloatWindowSource<T>
    where T: DirectAccess,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    pub fn new(source: T) -> Self {
        FloatWindowSource {
            source,
            start: 0,
            end: 0
        }
    }

    #[inline]
    fn sample_rate_for_all_channels(&self) -> usize {
        self.source.channels() as usize * self.source.sample_rate() as usize
    }
}

impl<T> Resettable for FloatWindowSource<T>
    where T: DirectAccess,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    fn reset(&mut self) {
        self.source.set_current(self.start);
    }
}

impl<T> FloatWindow for FloatWindowSource<T>
    where T: DirectAccess,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    fn play(&mut self, time: TimeSpan) {
        let multiplayer = self.sample_rate_for_all_channels();
        self.start = multiplayer * time.start as usize;
        self.end = self.start + (multiplayer * time.duration as usize);
        if self.source.current() < self.start || self.source.current() > self.end {
            self.source.set_current(self.start);
        }
    }

    #[inline]
    fn cursor(&self) -> usize {
        self.source.current()
    }

    #[inline]
    fn end(&self) -> usize {
        self.end
    }
}

impl<T> Source for FloatWindowSource<T>
    where T: DirectAccess,
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

impl<T> Deref for FloatWindowSource<T>
    where T: DirectAccess,
          <T as Iterator>::Item: Sample_
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<T> Iterator for FloatWindowSource<T>
    where T: DirectAccess,
          <T as Iterator>::Item: Sample_
{
    type Item = <T as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current() < self.end {
            self.source.next()
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.source.size_hint()
    }
}

impl<T> fmt::Debug for FloatWindowSource<T>
    where T: DirectAccess,
          <T as Iterator>::Item: Sample_
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FloatWindowSource")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("source", &self.source)
            .finish()
    }
}
