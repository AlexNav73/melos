
use imgui::*;

use player::Player;
use support_gfx::AppContext;
use state::{State, TimeFrame};
use console::Console;

pub struct MainWindow {
    state: State,
    player: Player,
    console: Console
}

impl AppContext for MainWindow {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        self.show_main_window(ui)
    }
}

impl MainWindow {
    pub fn new(state: State) -> Self {
        let player = Player::new(state.clone());
        player.open(state.path().to_str());
        MainWindow {
            console: Console::new(state.clone()),
            player,
            state,
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
                ui.input_text(im_str!(""), &mut self.state.lyrics_mut())
                    .multiline(ImVec2::new(550.0, 530.0))
                    .build();
                ui.next_column();
                ui.input_text(im_str!("song"), &mut self.state.path_mut()).build();
                ui.same_line(0.0);
                if ui.button(im_str!("open"), (0.0, 0.0)) {
                    self.state.clean_timings();
                    self.player.open(self.state.path().to_str());
                }
                if ui.button(im_str!("+"), (0.0, 0.0)) {
                    self.state.timings_mut().push(TimeFrame::default());
                }
                self.show_quatrains(ui);
                ui.spacing();
                self.player.show(ui);
                self.console.show(ui);
            });

        self.state.timings_mut().retain(|x| !x.remove);

        opened
    }

    fn show_quatrains<'a>(&mut self, ui: &Ui<'a>) {
        ui.child_frame(im_str!("quatrains"), (340.0, 200.0))
            .show_scrollbar(true)
            .show_borders(true)
            .build(|| {
                let mut play = None;
                for (idx, player) in self.state.timings_mut().iter_mut().enumerate() {
                    ui.with_id(idx as i32, || {
                        if ui.button(im_str!("X"), (30.0, 0.0)) {
                            player.remove = true;
                        }
                        ui.same_line(0.0);
                        let mut frame = [player.start, player.end];
                        ui.input_float2(im_str!(""), &mut frame)
                            .decimal_precision(2)
                            .build();
                        player.start = frame[0];
                        player.end = frame[1];
                        ui.same_line(0.0);
                        if ui.button(im_str!("play"), (35.0, 0.0)) {
                            play = Some((player.start, player.end));
                        }
                    });
                }
                play.map(|(x, y)| self.player.play(x, y));
            });
    }
}
