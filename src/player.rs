
use imgui::*;

use support_gfx::AppContext;
use state::State;
use song::{Song, TimeSpan};

pub struct Player {
    song: Song,
    state: State,
    start: f32,
    end: f32,
    volume: f32
}

impl Player {
    #[inline]
    pub fn new(state: State) -> Self {
        Player {
            song: Song::new(),
            volume: 50.0,
            start: 0.0,
            end: 0.0,
            state,
        }
    }

    #[inline]
    pub fn open<P: ToString>(&self, path: P) {
        self.song.open(path);
    }

    #[inline]
    pub fn update(&mut self, start: f32, end: f32) {
        self.start = start;
        self.end = end;
    }

    #[inline]
    pub fn play(&mut self) {
        self.song.play(TimeSpan::new(self.start(), self.duration()));
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
    fn start(&self) -> u32 {
        to_s(self.start)
    }

    #[inline]
    fn duration(&self) -> u32 {
        to_s(self.end) - self.start()
    }

    #[inline]
    fn progress(&self) -> f32 {
        let start = self.start();
        let begin = self.song.progress().checked_sub(start).unwrap_or(start);
        begin as f32 / self.duration() as f32
    }

    #[inline]
    fn log_load_status(&mut self) {
        if self.song.loaded() {
            self.state.log("Song was loaded".into());
        }
    }
}

fn to_s(time: f32) -> u32 {
    let decimal = time as u32;
    let real = ((time - decimal as f32) * 100.0) as u32;
    decimal * 60 + real
}

fn to_f(time: u32) -> f32 {
    let minutes = time / 60;
    let seconds = (time - minutes * 60) as f32 / 100.0;
    minutes as f32 + seconds
}

impl AppContext for Player {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        ui.child_frame(im_str!("player"), (340.0, 100.0))
            .show_borders(true)
            .build(|| {
                ui.progress_bar(self.progress())
                    .size((-1.0, 0.0))
                    .overlay_text(im_str!("{:.2}", to_f(self.song.progress())))
                    .build();
                ui.text(format!("{:.2}", self.start));
                ui.same_line(300.0);
                ui.text(format!("{:.2}", self.end));
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

