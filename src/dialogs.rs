
use failure::{Error, err_msg};
use imgui::*;
use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;

use std::fs::File;
use std::path::Path;
use serde_json;

use state::AppData;
use constants::*;
use configuration::CONFIG;

pub struct OpenFileDialog {
    path: ImString,
    cached_paths: Vec<ImString>,
    selected_item: i32,
}

pub enum OpenFileState {
    Displaying,
    Closed,
    Opened(AppData)
}

impl OpenFileDialog {
    pub fn new() -> Self {
        OpenFileDialog {
            path: ImString::with_capacity(MAX_PATH_LEN),
            cached_paths: enumerate_files(),
            selected_item: 0,
        }
    }

    pub fn show<'a>(&mut self, ui: &Ui<'a>) -> OpenFileState {
        let mut opened = true;
        let mut state = OpenFileState::Displaying;
        ui.window(im_str!("Open File"))
            .size(CONFIG.dialogs.dialog_sizes, ImGuiCond::Always)
            .opened(&mut opened)
            .collapsible(false)
            .resizable(false)
            .build(|| {
                ui.input_text(im_str!("##path"), &mut self.path).build();
                ui.same_line(0.0);
                if ui.button(im_str!("open"), (0.0, 0.0)) {
                    match read_state_from_file(self.path.to_str()) {
                        Ok(data) => state = OpenFileState::Opened(data),
                        Err(e) => println!("{}", e)
                    }
                }
                ui.with_item_width(CONFIG.dialogs.file_browser_width, || self.show_file_browser(ui));
            });

        if opened { state } else { OpenFileState::Closed }
    }

    pub fn update_cached_paths(&mut self) {
        self.cached_paths = enumerate_files();
    }

    fn show_file_browser<'a>(&mut self, ui: &Ui<'a>) {
        if self.cached_paths.is_empty() {
            self.update_cached_paths();
        }

        let old_selection = self.selected_item;
        let rpath = self.cached_paths.iter()
            .map(|x| x.as_ref())
            .collect::<Vec<_>>();

        ui.list_box(im_str!("##files"), &mut self.selected_item, rpath.as_slice(), 5);

        if let Some(ref p) = self.cached_paths.get(self.selected_item as usize) {
            if old_selection != self.selected_item ||
               self.path.to_str().is_empty() 
            {
                self.path.clear();
                self.path.push_str(p.as_ref());
            }
        }
    }
}

pub struct SaveFileDialog {
    path: ImString,
    cached_paths: Vec<ImString>,
}

impl SaveFileDialog {
    pub fn new() -> Self {
        SaveFileDialog {
            path: ImString::with_capacity(MAX_PATH_LEN),
            cached_paths: enumerate_files(),
        }
    }

    pub fn show<'a, F>(&mut self, ui: &Ui<'a>, get_data: F) -> bool 
        where F: FnOnce() -> AppData
    {
        let mut opened = true;
        let mut saved = false;
        ui.window(im_str!("Save File"))
            .size(CONFIG.dialogs.dialog_sizes, ImGuiCond::Always)
            .opened(&mut opened)
            .collapsible(false)
            .resizable(false)
            .build(|| {
                ui.input_text(im_str!("##path"), &mut self.path).build();
                ui.same_line(0.0);
                if ui.button(im_str!("save"), (0.0, 0.0)) {
                    match write_state_to_file(get_data(), self.path.to_str()) {
                        Ok(_) => {
                            saved = true;
                            println!("Project saved successfully");
                        },
                        Err(e) => println!("{}", e)
                    }
                    self.update_cached_paths();
                }
                ui.with_item_width(CONFIG.dialogs.file_browser_width, || self.show_file_browser(ui));
            });
        opened ^ saved
    }

    pub fn update_cached_paths(&mut self) {
        self.cached_paths = enumerate_files();
    }

    fn show_file_browser<'a>(&mut self, ui: &Ui<'a>) {
        if self.cached_paths.is_empty() {
            self.update_cached_paths();
        }

        let rpath = self.cached_paths.iter()
            .map(|x| x.as_ref())
            .collect::<Vec<_>>();

        ui.list_box(im_str!("##files"), &mut 0, rpath.as_slice(), 5);
    }
}

fn read_state_from_file<P: AsRef<Path>>(path: P) -> Result<AppData, Error> {
    use std::io::Read;

    let path = path.as_ref().with_extension(SAVE_FILE_EXT);
    ensure!(path.exists(), "Path is invalid");

    let mut file = File::open(path).map_err(|_| err_msg("Can't open file"))?;
    let metadata = file.metadata().map_err(|_| err_msg("Can't get file metadata"))?;
    let mut json = String::with_capacity(metadata.len() as usize);
    file.read_to_string(&mut json).map_err(|_| err_msg("Can't read save file"))?;
    serde_json::from_str::<AppData>(&json).map_err(|_| err_msg("Can't deserialize project data"))
}

fn write_state_to_file<P: AsRef<Path>>(state: AppData, path: P) -> Result<(), Error> {
    use std::io::Write;

    let path = path.as_ref().with_extension(SAVE_FILE_EXT);
    let mut file = File::create(path).map_err(|_| err_msg("Could not create file"))?;
    let json = serde_json::to_string(&state).map_err(|_| err_msg("Can't serialize project data"))?;
    file.write(json.as_bytes())
        .map(|_| ())
        .map_err(|_| err_msg("Can't save project to file"))
}

fn enumerate_files() -> Vec<ImString> {
    let filters = OverrideBuilder::new(&CONFIG.dialogs.base_dir)
        .add(SAVE_FILE_EXT_FILTER).expect("Save file filter is invalid")
        .build().expect("Can't build filters");

    WalkBuilder::new(&CONFIG.dialogs.base_dir)
        .max_depth(Some(1))
        .standard_filters(true)
        .overrides(filters)
        .build()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| ImString::new(e.path().to_str().expect("Can't cast path to str")))
        .collect()
}
