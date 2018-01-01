
use imgui::*;

use support_gfx::AppContext;
use song::Song;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(skip)]
    pub(crate) opened: bool,
    #[serde(skip)]
    pub(crate) is_deleted: bool,
    pub(crate) start: f32,
    pub(crate) end: f32,
    #[serde(skip)]
    song: Option<Song>
}

impl Player {
    pub fn new(song: Song, start: f32, end: f32) -> Self {
        Player { start, end, opened: false, is_deleted: false, song: Some(song) }
    }

    pub fn set_song(&mut self, song: Song) {
        self.song = Some(song);
    }

    pub fn update(&mut self, start: f32, end: f32) {
        self.start = start;
        self.end = end;
    }

    pub fn open(&mut self) {
        self.opened = true;
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
        let mut opened = self.opened;
        ui.window(im_str!("Player"))
            .size((100.0, 100.0), ImGuiCond::FirstUseEver)
            .opened(&mut opened)
            .collapsible(true)
            .build(|| {
                if ui.button(im_str!("Play"), (0.0, 0.0)) {
                    if let Some(ref song) = self.song {
                        song.play((self.start(), self.duration()));
                    }
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Stop"), (0.0, 0.0)) {
                    if let Some(ref song) = self.song {
                        song.stop();
                    }
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Pause"), (0.0, 0.0)) {
                    if let Some(ref song) = self.song {
                        song.pause();
                    }
                }
            });
        self.opened = opened;
        opened
    }
}

