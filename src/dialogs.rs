
use imgui::*;

use support_gfx::AppContext;
use state::State;

pub struct OpenFileDialog {
    pub opened: bool,
    load: bool,
    path: ImString,
    cached_paths: Vec<ImString>,
    selected_item: i32,
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
                    ui.input_text(im_str!("##path"), &mut self.path)
                        .build();
                    ui.same_line(0.0);
                    if ui.button(im_str!("open"), (0.0, 0.0)) {
                        self.load = self.state.open(self.path.to_str());
                    }
                    self.show_file_browser(ui);
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
            cached_paths: Vec::new(),
            selected_item: 0,
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

    fn show_file_browser<'a>(&mut self, ui: &Ui<'a>) {
        if self.cached_paths.is_empty() {
            self.cached_paths = enumerate_files();
        }

        let old_selection = self.selected_item;
        {
            let rpath = self.cached_paths.iter()
                .map(|x| x.as_ref())
                .collect::<Vec<_>>();

            if ui.list_box(im_str!("##files"),
                &mut self.selected_item,
                rpath.as_slice(), 5)
            {
            }
        }

        if let Some(ref p) = self.cached_paths.get(self.selected_item as usize) {
            if old_selection != self.selected_item {
                self.path.clear();
                self.path.push_str(p.as_ref());
            }
        }
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
                        self.state.save(self.path.to_str());
                    }
                });
        }
        self.opened = !(self.saved || !opened);
        self.opened
    }
}

fn enumerate_files() -> Vec<ImString> {
    vec![]
}
