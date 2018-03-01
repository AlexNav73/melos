
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate winit;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate rodio;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod player;
mod support_gfx;
mod main_window;
mod dialogs;
mod state;
mod console;
mod song;

use imgui::*;

use state::State;
use support_gfx::AppContext;
use main_window::MainWindow;
use dialogs::{OpenFileDialog, SaveFileDialog};

pub struct Program {
    state: State,
    open_file_dialog: OpenFileDialog,
    save_file_dialog: SaveFileDialog,
    main_window: Option<MainWindow>,
}

impl AppContext for Program {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        let mut opened = true;
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File"))
                .build(|| {
                    if ui.menu_item(im_str!("New")).build() {
                        self.main_window = Some(MainWindow::new(self.state.clone()));
                    }
                    if ui.menu_item(im_str!("Save")).build() {
                        self.save_file_dialog.opened = true;
                    }
                    if ui.menu_item(im_str!("Open")).build() {
                        self.open_file_dialog.opened = true;
                    }
                    if ui.menu_item(im_str!("Exit")).build() {
                        opened = false;
                    }
                });
        });

        self.open_file_dialog.show(ui);
        self.save_file_dialog.show(ui);

        if self.open_file_dialog.should_load() {
            self.main_window = Some(MainWindow::load(self.state.clone()));
        }

        let mut main_window_opened = false;
        if let Some(ref mut main_window) = self.main_window {
            main_window_opened = main_window.show(ui);
        }
        if !main_window_opened {
            self.main_window = None;
        }

        opened
    }
}

impl Program {
    fn new() -> Self {
        let state = State::new();
        Program {
            state: state.clone(),
            save_file_dialog: SaveFileDialog::new(state.clone()),
            open_file_dialog: OpenFileDialog::new(state),
            main_window: None,
        }
    }
}

fn main() {
    support_gfx::run("melos", Program::new());
}

