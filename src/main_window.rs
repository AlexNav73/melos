
use imgui::*;

use support_gfx::AppContext;
use player::Player;
use console::Console;
use dialogs::SaveFileDialog;
use state::{State, TimeFrame, ImLanguageTab};
use configuration::CONFIG;

pub struct MainWindow {
    state: State,
    player: Player,
    console: Console,
    save_file_dialog: Option<SaveFileDialog>,
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
            tooltip_input: ImString::with_capacity(CONFIG.main_window.tooltip_len),
            lang_name_buf: ImString::with_capacity(CONFIG.main_window.lang_name_len),
            save_file_dialog: None,
            language: 0,
            state,
        }
    }

    pub fn load(state: State) -> Self {
        let mut player = Player::new(state.clone());
        player.open(state.path().to_str());
        MainWindow {
            console: Console::new(state.clone()),
            tooltip_input: ImString::with_capacity(CONFIG.main_window.tooltip_len),
            lang_name_buf: ImString::with_capacity(CONFIG.main_window.lang_name_len),
            save_file_dialog: None,
            language: 0,
            player,
            state,
        }
    }

    fn show_main_window<'a>(&mut self, ui: &Ui<'a>) -> bool {
        let mut opened = true;
        ui.window(im_str!("Lyrics"))
            .size(CONFIG.main_window.main_window_size, ImGuiCond::FirstUseEver)
            .opened(&mut opened)
            .collapsible(false)
            .menu_bar(true)
            .build(|| {
                self.show_menu(ui);
                ui.columns(2, im_str!("##container"), false);
                ui.input_text(im_str!(""), &mut self.state.lyrics_mut()[self.language].text)
                    .multiline(ImVec2::new(
                            CONFIG.main_window.lyrics_input_width,
                            CONFIG.main_window.lyrics_input_height))
                    .build();
                ui.next_column();
                let column_idx = ui.get_column_index();
                ui.set_column_offset(column_idx, CONFIG.main_window.column_offset);
                ui.with_item_width(CONFIG.main_window.song_path_input_len, || {
                    ui.input_text(im_str!("##song"), &mut self.state.path_mut()).build();
                });
                ui.same_line(0.0);
                if ui.button(im_str!("open"), (0.0, 0.0)) {
                    self.player.open(self.state.path().to_str());
                }
                ui.with_item_width(CONFIG.main_window.timeframe_tooltip_width, || {
                    ui.input_text(im_str!("##tooltip"), &mut self.tooltip_input).build();
                });
                ui.same_line(0.0);
                if ui.button(im_str!("+"), (0.0, 0.0)) {
                    {
                        let tooltip = self.tooltip_input.to_str();
                        if tooltip.is_empty() {
                            self.state.timings_mut().push(TimeFrame::new());
                        } else {
                            self.state.timings_mut().push(TimeFrame::with_tooltip(tooltip));
                        }
                    }
                    self.tooltip_input.clear();
                }
                self.show_quatrains(ui);
                ui.spacing();
                self.player.show(ui);
                self.console.show(ui);
                self.show_save_file_dialog(ui);
            });

        self.state.timings_mut().retain(|x| !x.remove);

        opened
    }

    fn show_save_file_dialog<'a>(&mut self, ui: &Ui<'a>) {
        if let Some(mut sfd) = self.save_file_dialog.take() {
            if sfd.show(ui) {
                self.save_file_dialog = Some(sfd);
            }
        }
    }

    fn show_menu<'a>(&mut self, ui: &Ui<'a>) {
        ui.menu_bar(|| {
            ui.menu(im_str!("File")).build(|| {
                if ui.menu_item(im_str!("Save")).build() {
                    self.save_file_dialog = Some(SaveFileDialog::new(self.state.clone()));
                }
            });
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
                    ui.with_item_width(CONFIG.main_window.new_lang_input_width, || {
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
            if ui.button(im_str!("X"), (0.0, 0.0)) {
                self.state.lyrics_mut().remove(self.language);
                if self.state.lyrics().is_empty() {
                    self.state.lyrics_mut().push(ImLanguageTab::default());
                }
                self.language = 0;
            }
        });
    }

    fn show_quatrains<'a>(&mut self, ui: &Ui<'a>) {
        ui.child_frame(im_str!("quatrains"), CONFIG.main_window.quatrains_frame_size)
            .show_scrollbar(true)
            .show_borders(true)
            .build(|| {
                let mut play = None;
                for (idx, frame) in self.state.timings_mut().iter_mut().enumerate() {
                    ui.with_id(idx as i32, || {
                        if ui.button(im_str!("X"), (0.0, 0.0)) {
                            frame.remove = true;
                        }
                        ui.same_line(0.0);
                        let mut time_range = [frame.start, frame.end];
                        ui.input_float2(im_str!(""), &mut time_range)
                            .decimal_precision(2)
                            .build();
                        if ui.is_item_hovered() {
                            if let Some(ref t) = frame.tooltip {
                                ui.tooltip_text(t);
                            }
                        }
                        frame.start = time_range[0];
                        frame.end = time_range[1];
                        ui.same_line(0.0);
                        if ui.button(im_str!("play"), (0.0, 0.0)) {
                            play = Some((frame.start, frame.end));
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
