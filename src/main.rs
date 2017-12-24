
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate hound;
extern crate rodio;

mod song;
mod support_gfx;

use imgui::*;
use glutin::WindowEvent;

use song::Song;
use support_gfx::Program;

struct State {
    title: String,
    lyrics: ImString,
    timings: Vec<[f32; 2]>,
    path: ImString,
    song: Option<Song>
}

impl Default for State {
    fn default() -> Self {
        State {
            title: "melos".to_owned(),
            lyrics: ImString::with_capacity(2000),
            path: ImString::with_capacity(256),
            timings: Vec::new(),
            song: None
        }
    }
}

impl Program for State {
    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        self.show_lyrics(ui)
    }

    fn on_event(&mut self, _: WindowEvent) { }
}

impl State {
    fn show_lyrics<'a>(&mut self, ui: &Ui<'a>) -> bool {
        let mut opened = true;
        ui.window(im_str!("Lyrics"))
            .size((620.0, 565.0), ImGuiCond::FirstUseEver)
            .opened(&mut opened)
            .collapsible(false)
            .build(|| {
                ui.columns(2, im_str!("container"), false);
                ui.input_text(im_str!(""), &mut self.lyrics)
                    .multiline(ImVec2::new(400.0, 530.0))
                    .build();
                ui.next_column();
                ui.input_text(im_str!("song"), &mut self.path).build();
                ui.same_line(0.0);
                if ui.button(im_str!("Open"), (0.0, 0.0)) {
                    self.song = Some(Song::new(self.path.to_str()));
                }
                if ui.button(im_str!("+"), (0.0, 0.0)) {
                    self.timings.push([0.0, 0.0]);
                }
                if ui.button(im_str!("Stop"), (0.0, 0.0)) {
                    if let Some(ref song) = self.song {
                        song.stop();
                    }
                }
                let mut to_remove = Vec::new();
                let mut time_range = None;
                for (idx, interval) in self.timings.iter_mut().enumerate() {
                    ui.with_id(idx as i32, || {
                        if ui.button(im_str!("X"), (30.0, 0.0)) {
                            to_remove.push(idx);
                        }
                        ui.same_line(0.0);
                        ui.input_float2(im_str!(""), interval)
                            .decimal_precision(2)
                            .build();
                        ui.same_line(0.0);
                        if ui.button(im_str!("Play"), (40.0, 0.0)) {
                            let (start, duration) = to_secs(interval);
                            time_range = Some((start, duration));
                        }
                    });
                }
                for i in to_remove {
                    self.timings.remove(i);
                }
                if let Some((start, duration)) = time_range {
                    if let Some(ref song) = self.song {
                        song.play(start, duration);
                    }
                }
            });
        opened
    }
}

fn to_secs(interval: &[f32; 2]) -> (u32, u32) {
    let start = to_s(interval[0]);
    let end = to_s(interval[1]);
    (start, end - start)
}

fn to_s(time: f32) -> u32 {
    let decimal = time as u32;
    let real = ((time - decimal as f32) * 100.0) as u32;
    decimal * 60 + real
}

fn main() {
    State::default().run();
}

