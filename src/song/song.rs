
use rodio::{self, Source};
use failure::{Error, err_msg};

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::BufReader;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{channel, Receiver};

use super::{TimeSpan, FloatWindow, Inspectable};
use super::controls::Controls;
use super::sources::{
    SmartSource,
    BaseSource,
    FloatWindowSource,
    StoppableSource,
    PausableSource
};

pub enum SongMsg {
    Loaded,
    Failed(Error)
}

pub struct Song {
    controls: Arc<Controls>,
}

impl Song {
    pub fn new() -> Self {
        Song { controls: Arc::new(Controls::new()) }
    }

    #[allow(deprecated)]
    pub fn open<P: AsRef<Path>>(&self, path: P) -> Receiver<SongMsg> {
        use std::thread;

        let path: PathBuf = path.as_ref().into();
        let controls = self.controls.clone();
        let (tx, rx) = channel();

        thread::spawn(move || {
            let th = move || -> Result<(), Error> {
                ensure!(path.exists(), "File not found");

                let file = File::open(path)?;
                let decoder = rodio::Decoder::new(BufReader::new(file))?;

                let samples_rate = decoder.samples_rate();
                let channels = decoder.channels();
                let samples = decoder.collect::<Vec<_>>();
                let controls2 = controls.clone();

                let source = BaseSource::new(channels, samples_rate, samples);
                let source = FloatWindowSource::new(source);
                let source = SmartSource::new(source, controls.clone());
                let source = StoppableSource::new(source);
                let source = PausableSource::new(source)
                    .amplify(1.0)
                    .periodic_access(Duration::from_millis(5), move |src| {
                        src.inner_mut().stop(controls.stopped());
                        src.inner_mut().pause(controls.paused());
                        src.inner_mut().play(controls.time());
                        src.set_factor(controls.volume());
                    })
                .periodic_access(Duration::from_millis(995), move |src| {
                    controls2.set_progress(src.inner().inner().current_sec() as u32);
                })
                .convert_samples();

                let endpoint = rodio::get_endpoints_list()
                    .next()
                    .ok_or(err_msg("Can't get endpoints list"))?;
                Ok(rodio::play_raw(&endpoint, source))
            };

            match th() {
                Ok(_) => tx.send(SongMsg::Loaded).expect("Can't send signal"),
                Err(e) => tx.send(SongMsg::Failed(e)).expect("Can't send signal")
            }
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
}

impl Drop for Song {
    #[inline]
    fn drop(&mut self) {
        self.controls.stopped.store(true, Ordering::Relaxed);
    }
}
