
use serde_json;
use imgui::*;

use std::fs::File;
use std::path::Path;

use support_gfx::AppContext;
use state::State;

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
    state: State
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
                        self.load = false;
                        if let Ok(data) = self.open(self.path.to_str()) {
                            self.state.update_from_app_data(data);
                            self.load = true;
                        }
                    }
                });
        }
        self.opened = !(self.load || !opened);
        self.opened
    }
}

impl OpenFileDialog {
    pub fn new(state: State) -> Self {
        OpenFileDialog {
            opened: false,
            load: false,
            path: ImString::with_capacity(256),
            state
        }
    }

    pub fn should_load(&mut self) -> bool {
        if self.load {
            self.load = false;
            return true;
        }
        false
    }

    fn open<P: AsRef<Path>>(&self, path: P) -> Result<AppData, ()> {
        use std::io::Read;

        let mut file = File::open(path.as_ref()).map_err(|_| ())?;
        let mut json = String::with_capacity(file.metadata().unwrap().len() as usize);
        file.read_to_string(&mut json).map_err(|_| ())?;
        serde_json::from_str::<AppData>(&json).map_err(|_| ())
    }
}

pub struct SaveFileDialog {
    pub opened: bool,
    saved: bool,
    path: ImString,
    state: State
}

impl SaveFileDialog {
    pub fn new(state: State) -> Self {
        SaveFileDialog {
            opened: false,
            saved: false,
            path: ImString::with_capacity(256),
            state
        }
    }

    pub fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
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
                        let data = {
                            self.state.to_app_data()
                        };
                        self.save(data);
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
