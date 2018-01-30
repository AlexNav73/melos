
use serde_json;
use imgui::*;

use std::fs::File;
use std::path::Path;

use ::Program;
use support_gfx::AppContext;

#[derive(Serialize, Deserialize)]
pub struct AppData {
    pub lyrics: String,
    pub timings: Vec<(f32, f32)>,
    pub path: String
}

pub struct OpenFileDialog {
    pub opened: bool,
    load: bool,
    path: ImString,
}

impl AppContext for OpenFileDialog {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        let mut opened = self.opened;
        if opened {
            ui.window(im_str!("Open File"))
                .size((320.0, 265.0), ImGuiCond::FirstUseEver)
                .opened(&mut opened)
                .collapsible(false)
                .build(|| {
                    ui.input_text(im_str!("path"), &mut self.path)
                        .build();
                    if ui.button(im_str!("open"), (0.0, 0.0)) {
                        let file = Path::new(self.path.to_str());
                        self.load = file.exists() && file.is_file();
                    }
                });
        }
        self.opened = !(self.load || !opened);
        self.opened
    }
}

impl OpenFileDialog {
    pub fn new() -> Self {
        OpenFileDialog {
            opened: false,
            load: false,
            path: ImString::with_capacity(256)
        }
    }

    pub fn open(&mut self) -> Result<AppData, ()> {
        use std::io::Read;

        let mut file = File::open(self.path.to_str()).map_err(|_| ())?;
        let mut json = String::with_capacity(file.metadata().unwrap().len() as usize);
        file.read_to_string(&mut json).unwrap();
        Ok(serde_json::from_str::<AppData>(&json).expect("Invalid json file"))
    }

    pub fn should_load(&mut self) -> bool {
        if self.load {
            self.load = false;
            return true;
        }
        false
    }
}

type SaveFn = fn(&Program) -> Option<AppData>;

pub struct SaveFileDialog {
    pub opened: bool,
    saved: bool,
    path: ImString,
    on_save: SaveFn
}

impl SaveFileDialog {
    pub fn new(callback: SaveFn) -> Self {
        SaveFileDialog {
            opened: false,
            saved: false,
            path: ImString::with_capacity(256),
            on_save: callback
        }
    }

    pub fn show<'a>(&mut self, ui: &Ui<'a>, prog: &Program) -> bool {
        let mut opened = self.opened;
        if opened {
            ui.window(im_str!("Save File"))
                .size((320.0, 265.0), ImGuiCond::FirstUseEver)
                .opened(&mut opened)
                .collapsible(false)
                .build(|| {
                    ui.input_text(im_str!("path"), &mut self.path)
                        .build();
                    if ui.button(im_str!("save"), (0.0, 0.0)) {
                        if let Some(data) = (self.on_save)(prog) {
                            self.save(data);
                        }
                    }
                });
        }
        self.opened = !(self.saved || !opened);
        self.opened
    }

    fn save(&mut self, save_sate: AppData) {
        use std::io::Write;

        let mut file = File::create(self.path.to_str()).expect("Could not create file");
        file.write(serde_json::to_string(&save_sate).unwrap().as_bytes()).unwrap();
    }
}

