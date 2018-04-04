
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
extern crate ignore;
extern crate config;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

mod player;
mod support_gfx;
mod main_window;
mod dialogs;
mod state;
mod console;
mod song;
mod fonts;
mod constants;
mod configuration;

use imgui::*;

use support_gfx::AppContext;
use main_window::MainWindow;
use dialogs::{OpenFileDialog, OpenFileState};

pub struct Program {
    open_file_dialog: Option<OpenFileDialog>,
    main_window: Option<MainWindow>,
}

impl AppContext for Program {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        let mut opened = true;
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File"))
                .build(|| {
                    if ui.menu_item(im_str!("New")).build() {
                        self.main_window = Some(MainWindow::new());
                    }
                    if ui.menu_item(im_str!("Open")).build() {
                        self.open_file_dialog = Some(OpenFileDialog::new());
                    }
                    if ui.menu_item(im_str!("Exit")).build() {
                        opened = false;
                    }
                });
        });

        if let Some(mut ofd) = self.open_file_dialog.take() {
            match ofd.show(ui) {
                OpenFileState::Opened(data) => self.main_window = Some(MainWindow::load(data)),
                OpenFileState::Displaying => self.open_file_dialog = Some(ofd),
                OpenFileState::Closed => {}
            }
        }

        if let Some(mut window) = self.main_window.take() {
            if window.show(ui) {
                self.main_window = Some(window);
            }
        }

        opened
    }
}

impl Program {
    fn new() -> Self {
        Program {
            open_file_dialog: None,
            main_window: None,
        }
    }
}

fn main() {
    support_gfx::run("melos", Program::new());
}

