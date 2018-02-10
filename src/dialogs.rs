
use imgui::*;

use support_gfx::AppContext;
use state::State;

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
                        self.load = self.state.open(self.path.to_str());
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
