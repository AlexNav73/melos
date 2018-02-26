
use super::{TimeSpan, FloatWindow, Sample};
use super::sources::{SmartSource, BaseSource, FloatWindowSource};
use super::controls::Controls;

use rodio;
use rodio::Source;

use std::fs::File;
use std::path::{Path,PathBuf};
use std::io::BufReader;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{channel, Receiver};

pub struct Song {
    controls: Arc<Controls>,
}

impl Song {
    pub fn new() -> Self {
        Song { controls: Arc::new(Controls::new()) }
    }

    #[allow(deprecated)]
    pub fn open<P: AsRef<Path>>(&self, path: P) -> Receiver<Arc<Vec<Sample>>> {
        use std::thread;

        let path: PathBuf = path.as_ref().into();
        let controls = self.controls.clone();
        let (tx, rx) = channel();

        thread::spawn(move || {
            let file = File::open(path).expect("Invalid file name");
            let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();

            let samples_rate = decoder.samples_rate();
            let channels = decoder.channels();
            let samples = Arc::new(decoder.collect::<Vec<_>>());
            let controls2 = controls.clone();
            let controls3 = controls.clone();

            let base = BaseSource::new(channels, samples_rate, samples.clone());
            let float_window = FloatWindowSource::new(base);
            let source = SmartSource::new(float_window, controls.clone())
                .amplify(1.0)
                .periodic_access(Duration::from_millis(5), move |src| {
                    src.inner_mut().stop(controls.stopped());
                    src.inner_mut().pause(controls.paused());
                    src.inner_mut().play(controls.time());
                    src.set_factor(controls.volume());
                })
                .periodic_access(Duration::from_millis(995), move |src| {
                    controls2.set_progress(src.inner().inner().cursor() as u32);
                })
                .convert_samples();

            let endpoint = rodio::get_endpoints_list().next().unwrap();
            rodio::play_raw(&endpoint, source);

            controls3.loaded.store(true, Ordering::SeqCst);
            tx.send(samples).unwrap();
        });

        rx
    }

    #[inline]
    pub fn play(&self, time: TimeSpan) {
        self.controls.set_time(time);
        self.controls.set_stopped(false);
        self.controls.set_paused(false);
    }

    #[inline]
    pub fn stop(&self) {
        self.controls.set_stopped(true);
    }

    #[inline]
    pub fn pause(&self) {
        self.controls.set_paused(true);
    }

    #[inline]
    pub fn volume(&mut self, value: f32) {
        self.controls.set_volume(value);
    }

    #[inline]
    pub fn progress(&self) -> u32 {
        self.controls.progress()
    }

    #[inline]
    pub fn loaded(&self) -> bool {
        self.controls.loaded()
    }
}

impl Drop for Song {
    #[inline]
    fn drop(&mut self) {
        self.controls.stopped.store(true, Ordering::Relaxed);
    }
}
