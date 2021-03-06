
use super::{FloatWindow, Resettable};
use super::controls::Controls;

use rodio::{Source, Sample as Sample_};

use std::fmt;
use std::time::Duration;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};

pub struct SmartSource<T>
    where T: FloatWindow,
          <T as Iterator>::Item: Sample_
{
    source: T,
    controls: Arc<Controls>
}

impl<T> SmartSource<T> 
    where T: FloatWindow,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    pub fn new(source: T, controls: Arc<Controls>) -> Self {
        SmartSource {
            source,
            controls,
        }
    }
}


impl<T> Resettable for SmartSource<T> 
    where T: FloatWindow,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    fn reset(&mut self) {
        self.source.reset();
    }
}

impl<T> Iterator for SmartSource<T>
    where T: FloatWindow,
          <T as Iterator>::Item: Sample_
{
    type Item = <T as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let a @ Some(_) = self.source.next() {
            a
        } else {
            self.controls.set_stopped(true);
            return Some(<T as Iterator>::Item::zero_value());
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.source.size_hint()
    }
}

impl<T> fmt::Debug for SmartSource<T>
    where T: FloatWindow,
          <T as Iterator>::Item: Sample_
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SmartSource")
            .field("source", &self.source)
            .finish()
    }
}

impl<T> Source for SmartSource<T>
    where T: FloatWindow,
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

impl<T> Deref for SmartSource<T>
    where T: FloatWindow,
          <T as Iterator>::Item: Sample_
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<T> DerefMut for SmartSource<T>
    where T: FloatWindow,
          <T as Iterator>::Item: Sample_
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source
    }
}

