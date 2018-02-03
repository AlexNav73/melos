
use std::rc::Rc;
use std::cell::RefCell;

use imgui::*;

use player::Player;
use support_gfx::AppContext;
use dialogs::AppData;
use state::State;

pub struct MainWindow {
    state: Rc<RefCell<State>>,
    player: Player,
}

impl AppContext for MainWindow {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        self.show_main_window(ui)
    }
}

impl MainWindow {
    pub fn new(state: Rc<RefCell<State>>) -> Self {
        let player = Player::new();
        player.open(state.borrow().path().to_str());
        MainWindow {
            player,
            state
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
                ui.input_text(im_str!(""), self.state.borrow_mut().lyrics_mut())
                    .multiline(ImVec2::new(700.0, 530.0))
                    .build();
                ui.next_column();
                ui.input_text(im_str!("song"), self.state.borrow_mut().path_mut()).build();
                ui.same_line(0.0);
                if ui.button(im_str!("open"), (0.0, 0.0)) {
                    self.state.borrow_mut().clean_timings();
                    self.player.open(self.state.borrow().path().to_str());
                }
                if ui.button(im_str!("+"), (0.0, 0.0)) {
                    self.state.borrow_mut().timings_mut().push(([0.0, 0.0], false));
                }
                self.show_quatrains(ui);
                ui.spacing();
                self.player.show(ui);
            });

        self.state.borrow_mut().timings_mut().retain(|x| !x.1);

        opened
    }

    fn show_quatrains<'a>(&mut self, ui: &Ui<'a>) {
        ui.child_frame(im_str!("quatrains"), (340.0, 200.0))
            .show_scrollbar(true)
            .show_borders(true)
            .build(|| {
                let mut play = None;
                for (idx, player) in self.state.borrow_mut().timings_mut().iter_mut().enumerate() {
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
}
