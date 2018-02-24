
mod song;
mod source;
mod controls;

pub use self::song::*;

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

