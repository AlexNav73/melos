
use imgui::*;

use support_gfx::AppContext;
use song::Song;

pub struct Player {
    start: f32,
    end: f32,
    song: Option<Song>
}

impl Player {
    pub fn new() -> Self {
        Player { start: 0.0, end: 0.0, song: None }
    }

    pub fn set_song(&mut self, song: Song) {
        self.song = Some(song);
    }

    pub fn update(&mut self, start: f32, end: f32) {
        self.start = start;
        self.end = end;
    }

    pub fn play(&self) {
        if let Some(ref song) = self.song {
            song.play((self.start(), self.duration()));
        }
    }

    pub fn stop(&self) {
        if let Some(ref song) = self.song {
            song.stop();
        }
    }

    pub fn pause(&self) {
        if let Some(ref song) = self.song {
            song.pause();
        }
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
                if ui.button(im_str!("Play"), (0.0, 0.0)) {
                    self.play();
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Stop"), (0.0, 0.0)) {
                    self.stop();
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Pause"), (0.0, 0.0)) {
                    self.pause();
                }
            });
        true
    }
}

