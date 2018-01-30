
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
mod main_window;
mod dialogs;

use imgui::*;

use support_gfx::AppContext;
use main_window::MainWindow;
use dialogs::{OpenFileDialog, SaveFileDialog, AppData};

pub struct Program {
    //open_file_dialog: OpenFileDialog,
    //save_file_dialog: SaveFileDialog,
    main_window: Option<MainWindow>,
}

impl AppContext for Program {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        let mut opened = true;
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File"))
                .build(|| {
                    if ui.menu_item(im_str!("Save")).build() {
                        //self.save_file_dialog.opened = true;
                    }
                    if ui.menu_item(im_str!("Open")).build() {
                        //self.open_file_dialog.opened = true;
                    }
                    if ui.menu_item(im_str!("Exit")).build() {
                        opened = false;
                    }
                });
        });

        //self.open_file_dialog.show(ui);
        //self.save_file_dialog.show(ui, self);

        //if self.open_file_dialog.should_load() {
            //if let Ok(saved_state) = self.open_file_dialog.open() {
                //self.main_window = Some(MainWindow::new(saved_state));
            //}
        //}
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
        Program {
            //save_file_dialog: SaveFileDialog::new(Program::on_save),
            //open_file_dialog: OpenFileDialog::new(),
            main_window: Some(MainWindow::new())
        }
    }

    fn on_save(&self) -> Option<AppData> {
        self.main_window.as_ref().map(|x| x.on_save())
    }
}

fn main() {
    support_gfx::run("melos", Program::new());
}

