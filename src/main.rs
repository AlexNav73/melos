
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate hound;

mod song;
mod support_gfx;

use imgui::*;

use song::Song;

struct State {
    lyrics: ImString
}

impl Default for State {
    fn default() -> Self {
        State {
            lyrics: ImString::with_capacity(2000)
        }
    }
}

fn main() {
    //let mut song = Song::new("samples/sonne.wav");
    //song.split_by(&[(27, 55), (84, 115)], "samples/sonna.wav");
    let mut state = State::default();

    support_gfx::run(
        "melos".to_owned(),
        |ui| show(ui, &mut state));
}

fn show<'a>(ui: &Ui<'a>, state: &mut State) -> bool {
    let mut opened = true;
    ui.window(im_str!("Hello world"))
        .size((100.0, 100.0), ImGuiCond::FirstUseEver)
        .opened(&mut opened)
        .build(|| {
            ui.input_text(im_str!("Lyrics"), &mut state.lyrics)
              .multiline(ImVec2::new(500.0, 500.0))
              .build();
        });
    opened
}

