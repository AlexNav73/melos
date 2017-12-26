
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate hound;
extern crate rodio;

mod song;
mod player;
mod support_gfx;

use imgui::*;

use song::Song;
use player::Player;
use support_gfx::AppContext;

struct State {
    lyrics: ImString,
    timings: Vec<Player>,
    path: ImString,
    song: Option<Song>
}

impl Default for State {
    fn default() -> Self {
        State {
            lyrics: ImString::with_capacity(2000),
            path: ImString::with_capacity(256),
            timings: Vec::new(),
            song: None
        }
    }
}

impl AppContext for State {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        self.show_lyrics(ui)
    }
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
                    self.timings.push(Player::new(0.0, 0.0));
                }
                if ui.button(im_str!("Stop"), (0.0, 0.0)) {
                    if let Some(ref mut song) = self.song {
                        song.stop();
                    }
                }
                let mut active_player = None;
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
                            active_player = Some(player);
                        }
                    });
                }
                if let Some(player) = active_player {
                    if let Some(ref song) = self.song {
                        player.opened = true;
                        song.play(player);
                    }
                }
            });

        self.timings.retain(|x| !x.is_deleted);
        self.timings.iter_mut()
            .filter(|x| x.opened)
            .for_each(|x| { x.show(ui); });
        opened
    }
}

fn main() {
    support_gfx::run("melos", State::default());
}

