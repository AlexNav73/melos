
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
    timings: Vec<Player>,
    path: ImString,
    song: Option<Song>,
}

impl Default for State {
    fn default() -> Self {
        State {
            lyrics: ImString::with_capacity(2000),
            path: ImString::with_capacity(256),
            timings: Vec::new(),
            song: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AppData {
    lyrics: String,
    timings: Vec<Player>,
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
                if ui.button(im_str!("Open"), (0.0, 0.0)) {
                    self.timings = Vec::new();
                    self.song = Some(Song::new(self.path.to_str()));
                }
                if ui.button(im_str!("+"), (0.0, 0.0)) {
                    if let Some(ref song) = self.song {
                        self.timings.push(Player::new(song.clone(), 0.0, 0.0));
                    }
                }
                for (idx, player) in self.timings.iter_mut().enumerate() {
                    ui.with_id(idx as i32, || {
                        if ui.button(im_str!("X"), (30.0, 0.0)) {
                            player.is_deleted = true;
                        }
                        ui.same_line(0.0);
                        let mut interval = [player.start, player.end];
                        ui.input_float2(im_str!(""), &mut interval)
                            .decimal_precision(2)
                            .build();
                        player.update(interval[0], interval[1]);
                        ui.same_line(0.0);
                        if ui.button(im_str!("Play"), (40.0, 0.0)) {
                            player.play();
                        }
                    });
                }
            });

        self.timings.retain(|x| !x.is_deleted);
        self.timings.iter_mut()
            .filter(|x| x.opened)
            .for_each(|x| { x.show(ui); });
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

    fn save(&self) {
        use std::io::Write;

        let data = AppData {
            lyrics: self.lyrics.to_str().to_owned(),
            timings: self.timings.as_slice().to_vec(),
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
        let data = serde_json::from_str::<AppData>(&json).expect("Invalid json file");
        let song = Song::new(data.path.as_str());

        self.lyrics = ImString::new(data.lyrics);
        self.timings = data.timings;
        self.path = ImString::new(data.path);
        self.song = Some(song.clone());

        for x in &mut self.timings {
            x.set_song(song.clone());
        }
    }
}

fn main() {
    support_gfx::run("melos", State::default());
}

