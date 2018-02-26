
use imgui::*;

use player::Player;
use support_gfx::AppContext;
use state::{State, TimeFrame, ImLanguageTab};
use console::Console;

pub struct MainWindow {
    state: State,
    player: Player,
    console: Console,
    tooltip_input: ImString,
    language: usize,
    lang_name_buf: ImString
}

impl AppContext for MainWindow {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        self.show_main_window(ui)
    }
}

impl MainWindow {
    pub fn new(state: State) -> Self {
        MainWindow {
            console: Console::new(state.clone()),
            player: Player::new(state.clone()),
            tooltip_input: ImString::with_capacity(15),
            lang_name_buf: ImString::with_capacity(5),
            language: 0,
            state,
        }
    }

    pub fn load(state: State) -> Self {
        let player = Player::new(state.clone());
        // TODO(alex): Find a better way to handle `borrow already borrowed content` error
        let path = { state.path().to_str().to_owned() };
        player.open(path);
        MainWindow {
            console: Console::new(state.clone()),
            tooltip_input: ImString::with_capacity(15),
            lang_name_buf: ImString::with_capacity(5),
            language: 0,
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
            .menu_bar(true)
            .build(|| {
                self.show_menu(ui);
                ui.columns(2, im_str!("##container"), false);
                ui.input_text(im_str!(""), &mut self.state.lyrics_mut()[self.language].text)
                    .multiline(ImVec2::new(550.0, 530.0))
                    .build();
                ui.next_column();
                let column_idx = ui.get_column_index();
                ui.set_column_offset(column_idx, 560.0);
                ui.with_item_width(300.0, || {
                    ui.input_text(im_str!("##song"), &mut self.state.path_mut()).build();
                });
                ui.same_line(0.0);
                if ui.button(im_str!("open"), (0.0, 0.0)) {
                    self.player.open(self.state.path().to_str());
                }
                ui.with_item_width(100.0, || {
                    ui.input_text(im_str!("##tooltip"), &mut self.tooltip_input).build();
                });
                ui.same_line(0.0);
                if ui.button(im_str!("+"), (0.0, 0.0)) {
                    self.state.timings_mut().push(TimeFrame::new(self.tooltip_input.to_str()));
                    self.tooltip_input.clear();
                }
                self.show_quatrains(ui);
                ui.spacing();
                self.player.show(ui);
                self.console.show(ui);
            });

        self.state.timings_mut().retain(|x| !x.remove);

        opened
    }

    fn show_menu<'a>(&mut self, ui: &Ui<'a>) {
        ui.menu_bar(|| {
            ui.menu(im_str!("Languages")).build(|| {
                let mut lang_id = self.language;
                for (idx, tab) in self.state.lyrics().iter().enumerate() {
                    ui.with_id(idx as i32, || {
                        if ui.menu_item(&tab.lang).build() {
                            lang_id = idx;
                        }
                    });
                }
                self.language = lang_id;
                ui.menu(im_str!("New")).build(|| {
                    ui.with_item_width(40.0, || {
                        ui.input_text(im_str!("##new_lang"), &mut self.lang_name_buf)
                            .build();
                    });
                    ui.same_line(0.0);
                    if ui.button(im_str!("+"), (0.0, 0.0)) {
                        let tab = ImLanguageTab::new(self.lang_name_buf.to_str(), "");
                        self.state.lyrics_mut().push(tab);
                        self.lang_name_buf.clear();
                    }
                });
            });
        });
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
                        if ui.is_item_hovered() {
                            ui.tooltip_text(&player.tooltip);
                        }
                        player.start = frame[0];
                        player.end = frame[1];
                        ui.same_line(0.0);
                        if ui.button(im_str!("play"), (35.0, 0.0)) {
                            play = Some((player.start, player.end));
                        }
                    });
                }
                play.map(|(x, y)| {
                    self.player.update(x, y);
                    self.player.play();
                });
            });
    }
}
