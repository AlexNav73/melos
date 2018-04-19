
use imgui::*;

pub struct Console {
    logs: Vec<String>
}

impl Console {
    pub fn new() -> Self {
        Console { logs: Vec::new() }
    }

    pub fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        const DISTANCE: f32 = 10.0;
        const CORNER: i32 = 0;

        let (display_size_x, display_size_y) = ui.imgui().display_size();
        let window_pos = (if CORNER & 1 > 0 { display_size_x - DISTANCE } else { DISTANCE }, if CORNER & 2 > 0 { display_size_y - DISTANCE } else { DISTANCE });
        let window_pos_pivot = (if CORNER & 1 > 0 { 1.0 } else { 0.0 }, if CORNER & 2 > 0 { 1.0 } else { 0.0 });

        let mut opened = false;
        ui.window(im_str!("##logs"))
            .position(window_pos, ImGuiCond::Always)
            .size(window_pos_pivot, ImGuiCond::Always)
            .opened(&mut opened)
            //.collapsible(false)
            //.menu_bar(false)
            //.title_bar(false)
            //.resizable(false)
            .always_auto_resize(true)
            //.movable(false)
            //.save_settings(false)
            .no_focus_on_appearing(true)
            //.menu_bar(false)
            //.alpha(0.3)
            .build(|| {
                self.logs.iter().for_each(|log| ui.text(log));
            });

        opened
    }
}

