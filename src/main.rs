
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate hound;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod song;
mod support_gfx;

use imgui::*;
use glutin::WindowEvent;
use regex::Regex;

use song::Song;
use support_gfx::Program;

lazy_static! {
    static ref SPLIT: Regex = Regex::new(r##"\[([0-9:]+)\]([^0-9:]*)\[([0-9:]+)\]"##).unwrap();
}

struct State {
    title: String,
    lyrics: ImString
}

impl Default for State {
    fn default() -> Self {
        State {
            title: "melos".to_owned(),
            lyrics: ImString::with_capacity(2000)
        }
    }
}

impl Program for State {
    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        let mut opened = true;
        ui.window(im_str!("Hello world"))
            .size((420.0, 565.0), ImGuiCond::Once)
            .opened(&mut opened)
            .build(|| {
                ui.input_text(im_str!(""), &mut self.lyrics)
                    .multiline(ImVec2::new(400.0, 530.0))
                    .build();
            });
        opened
    }

    fn on_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::ReceivedCharacter(c) => {
                for captures in  SPLIT.captures_iter(self.lyrics.to_str()) {
                    let start = captures.get(1).map_or("", |m| m.as_str());
                    let text = captures.get(2).map_or("", |m| m.as_str());
                    let end = captures.get(3).map_or("", |m| m.as_str());

                    println!("START: {:?}\nTEXT: {:?}\nEND: {:?}", start, text, end);
                }
            },
            _ => {}
        }
    }
}

fn main() {
    //let mut song = Song::new("samples/sonne.wav");
    //song.split_by(&[(27, 55), (84, 115)], "samples/sonna.wav");

    State::default().run();
}

