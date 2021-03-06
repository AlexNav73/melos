
use std::path::Path;
use std::sync::mpsc::Receiver;

use imgui::*;

use support_gfx::AppContext;
use song::{Song, SongMsg, TimeSpan};
use configuration::CONFIG;
use console::Logger;

pub struct Player {
    logger: Logger,
    song: Song,
    time_span: TimeSpan,
    volume: f32,
    loaded_event: Option<Receiver<SongMsg>>
}

impl Player {
    #[inline]
    pub fn new(logger: Logger) -> Self {
        Player {
            logger,
            song: Song::new(),
            volume: CONFIG.player.default_volume,
            time_span: TimeSpan::default(),
            loaded_event: None,
        }
    }

    #[inline]
    pub fn open<P: AsRef<Path>>(&mut self, path: P) {
        self.loaded_event = Some(self.song.open(path));
    }

    #[inline]
    pub fn update(&mut self, time_span: TimeSpan) {
        self.time_span = time_span;
    }

    #[inline]
    pub fn play(&mut self) {
        self.song.play(self.time_span);
    }

    #[inline]
    pub fn stop(&self) {
        self.song.stop();
    }

    #[inline]
    pub fn pause(&self) {
        self.song.pause();
    }

    #[inline]
    pub fn update_volume(&mut self) {
        self.song.volume(self.volume / 100.0);
    }

    #[inline]
    fn start(&mut self) -> f32 {
        to_f(self.time_span.start)
    }

    #[inline]
    fn end(&mut self) -> f32 {
        to_f(self.time_span.start + self.time_span.duration)
    }

    #[inline]
    fn progress(&self) -> f32 {
        let start = self.time_span.start;
        let begin = self.song.progress().checked_sub(start).unwrap_or(start);
        begin as f32 / self.time_span.duration as f32
    }

    #[inline]
    fn log_load_status(&mut self) {
        if let Some(ref e) = self.loaded_event {
            if let Ok(msg) = e.try_recv() {
                match msg {
                    SongMsg::Loaded => self.logger.log("Song was loaded"),
                    SongMsg::Failed(e) => self.logger.log(format!("{}", e))
                }
            }
        }
    }
}

fn to_f(time: u32) -> f32 {
    let minutes = time / 60;
    let seconds = (time - minutes * 60) as f32 / 100.0;
    minutes as f32 + seconds
}

impl AppContext for Player {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        ui.child_frame(im_str!("player"), CONFIG.player.player_frame_size)
            .show_borders(true)
            .build(|| {
                ui.progress_bar(self.progress())
                    .size((-1.0, 0.0))
                    .overlay_text(im_str!("{:.2}", to_f(self.song.progress())))
                    .build();
                ui.text(format!("{:.2}", self.start()));
                ui.same_line(300.0);
                ui.text(format!("{:.2}", self.end()));
                ui.slider_float(im_str!("volume"), &mut self.volume, 0.0, 100.0)
                    .display_format(im_str!("%.0f"))
                    .build();
                if ui.button(im_str!("play"), (0.0, 0.0)) {
                    self.play();
                }
                ui.same_line(0.0);
                if ui.button(im_str!("stop"), (0.0, 0.0)) {
                    self.stop();
                }
                ui.same_line(0.0);
                if ui.button(im_str!("pause"), (0.0, 0.0)) {
                    self.pause();
                }
            });

        self.update_volume();
        self.log_load_status();

        true
    }
}

