
use imgui::*;

use support_gfx::AppContext;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(skip)]
    pub(crate) opened: bool,
    #[serde(skip)]
    pub(crate) is_deleted: bool,
    pub(crate) start: f32,
    pub(crate) end: f32
}

impl Player {
    pub fn new(start: f32, end: f32) -> Self {
        Player { start, end, opened: false, is_deleted: false }
    }

    pub fn update(&mut self, start: f32, end: f32) {
        self.start = start;
        self.end = end;
    }

    pub fn start(&self) -> u32 {
        to_s(self.start)
    }

    pub fn duration(&self) -> u32 {
        to_s(self.end) - self.start()
    }
}

fn to_s(time: f32) -> u32 {
    let decimal = time as u32;
    let real = ((time - decimal as f32) * 100.0) as u32;
    decimal * 60 + real
}

impl AppContext for Player {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        let mut opened = self.opened;
        ui.window(im_str!("Player"))
            .size((100.0, 100.0), ImGuiCond::FirstUseEver)
            .opened(&mut opened)
            .collapsible(true)
            .build(|| {
                ui.text(im_str!("player"));
            });
        self.opened = opened;
        opened
    }
}

