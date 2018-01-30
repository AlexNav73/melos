
use imgui::*;

use support_gfx::AppContext;
use song::{Song, TimeSpan};

pub struct Player {
    start: f32,
    end: f32,
    song: Song,
    volume: f32
}

impl Player {
    pub fn new() -> Self {
        Player {
            song: Song::new(),
            volume: 50.0,
            start: 0.0,
            end: 0.0
        }
    }

    pub fn update(&mut self, start: f32, end: f32) {
        self.start = start;
        self.end = end;
    }

    pub fn open<P: ToString>(&self, path: P) {
        self.song.open(path);
    }

    pub fn play(&self) {
        self.song.play(TimeSpan::new(self.start(), self.duration()));
    }

    pub fn stop(&self) {
        self.song.stop();
    }

    pub fn pause(&self) {
        self.song.pause();
    }

    pub fn volume(&mut self) {
        self.song.volume(self.volume / 100.0);
    }

    fn start(&self) -> u32 {
        to_s(self.start)
    }

    fn duration(&self) -> u32 {
        to_s(self.end) - self.start()
    }
}

fn to_s(time: f32) -> u32 {
    let decimal = time as u32;
    let real = ((time - decimal as f32) * 100.0) as u32;
    decimal * 60 + real
}

impl AppContext for Player {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        ui.child_frame(im_str!("player"), (340.0, 200.0))
            .build(|| {
                ui.slider_float(im_str!("volume"), &mut self.volume, 0.0, 100.0)
                    .display_format(im_str!("%.0f"))
                    .build();
                self.volume();
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
        true
    }
}

