
use imgui::*;

use support_gfx::AppContext;
use configuration::CONFIG;

pub struct Console {
}

impl Console {
    pub fn new() -> Self {
        Console {}
    }
}

impl AppContext for Console {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        ui.child_frame(im_str!("logs"), CONFIG.console.console_size)
            .show_scrollbar(true)
            .show_borders(true)
            .build(|| {
                // TODO: Implement using channels
                // .iter().for_each(|log| ui.text(log));
            });
        true
    }
}

