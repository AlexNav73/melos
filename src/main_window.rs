
use imgui::*;

use song::Song;
use player::Player;
use support_gfx::AppContext;
use dialogs::AppData;

pub struct MainWindow {
    lyrics: ImString,
    timings: Vec<([f32; 2], bool)>,
    path: ImString,
    song: Song,
    player: Player,
}

impl AppContext for MainWindow {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        self.show_main_window(ui)
    }
}

impl MainWindow {
    pub fn new(mut saved_state: AppData) -> Self {
        let song = Song::new(saved_state.path.as_str());
        MainWindow {
            lyrics: ImString::new(saved_state.lyrics),
            path: ImString::new(saved_state.path),
            timings: saved_state.timings.drain(..).map(|(x, y)| ([x, y], false)).collect(),
            song: song.clone(),
            player: Player::new(song),
        }
    }

    fn show_main_window<'a>(&mut self, ui: &Ui<'a>) -> bool {
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
                if ui.button(im_str!("open"), (0.0, 0.0)) {
                    self.timings = Vec::new();
                    self.song = Song::new(self.path.to_str());
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

    pub fn on_save(&self) -> AppData {
        AppData {
            lyrics: self.lyrics.to_str().to_owned(),
            timings: self.timings.iter().map(|&(x, _)| (x[0], x[1])).collect(),
            path: self.path.to_str().to_owned()
        }
    }
}
