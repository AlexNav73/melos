
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate rodio;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod song;
mod player;
mod support_gfx;

use std::fs::File;

use imgui::*;

use song::Song;
use player::Player;
use support_gfx::AppContext;

const SAVE_FILE: &str = "save.json";

struct State {
    lyrics: ImString,
    timings: Vec<([f32; 2], bool)>,
    path: ImString,
    song: Option<Song>,
    player: Player
}

impl Default for State {
    fn default() -> Self {
        State {
            lyrics: ImString::with_capacity(2000),
            path: ImString::with_capacity(256),
            timings: Vec::new(),
            song: None,
            player: Player::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AppData {
    lyrics: String,
    timings: Vec<(f32, f32)>,
    path: String
}

impl AppContext for State {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        self.show_lyrics(ui)
    }
}

impl State {
    fn show_lyrics<'a>(&mut self, ui: &Ui<'a>) -> bool {
        let mut opened = true;
        self.show_main_menu(ui);
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
                if ui.button(im_str!("open"), (0.0, 0.0)) {
                    self.timings = Vec::new();
                    self.song = Some(Song::new(self.path.to_str()));
                }
                if ui.button(im_str!("+"), (0.0, 0.0)) {
                    self.timings.push(([0.0, 0.0], false));
                }
                self.show_quatrains(ui);
                ui.spacing();
                self.player.show(ui);
            });

        self.timings.retain(|x| !x.1);

        opened
    }

    fn show_main_menu<'a>(&mut self, ui: &Ui<'a>) {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File"))
              .build(|| {
                  if ui.menu_item(im_str!("Save")).build() {
                      self.save();
                  }
                  if ui.menu_item(im_str!("Open")).build() {
                      self.open();
                  }
              });
        });
    }

    fn show_quatrains<'a>(&mut self, ui: &Ui<'a>) {
        ui.child_frame(im_str!("quatrains"), (340.0, 200.0))
            .show_scrollbar(true)
            .show_borders(true)
            .build(|| {
                let mut play = None;
                for (idx, player) in self.timings.iter_mut().enumerate() {
                    ui.with_id(idx as i32, || {
                        if ui.button(im_str!("X"), (30.0, 0.0)) {
                            player.1 = true;
                        }
                        ui.same_line(0.0);
                        ui.input_float2(im_str!(""), &mut player.0)
                            .decimal_precision(2)
                            .build();
                        ui.same_line(0.0);
                        if ui.button(im_str!("@>"), (35.0, 0.0)) {
                            play = Some((player.0[0], player.0[1]));
                        }
                    });
                }
                play.map(|(x, y)| self.player.update(x, y));
            });
    }

    fn save(&self) {
        use std::io::Write;

        let data = AppData {
            lyrics: self.lyrics.to_str().to_owned(),
            timings: self.timings.iter().map(|&(x, _)| (x[0], x[1])).collect(),
            path: self.path.to_str().to_owned()
        };

        let mut file = File::create(SAVE_FILE).expect("Could not create file");
        file.write(serde_json::to_string(&data).unwrap().as_bytes()).unwrap();
    }

    fn open(&mut self) {
        use std::io::Read;

        let mut file = File::open(SAVE_FILE).expect("Could not open file");
        let mut json = String::with_capacity(file.metadata().unwrap().len() as usize);
        file.read_to_string(&mut json).unwrap();
        let mut data = serde_json::from_str::<AppData>(&json).expect("Invalid json file");
        let song = Song::new(data.path.as_str());

        self.lyrics = ImString::new(data.lyrics);
        self.timings = data.timings.drain(..).map(|(x, y)| ([x, y], false)).collect();
        self.path = ImString::new(data.path);
        self.song = Some(song.clone());

        self.player.set_song(song);
    }
}

fn main() {
    support_gfx::run("melos", State::default());
}

