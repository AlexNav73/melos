
use imgui::*;

use support_gfx::AppContext;
use state::State;

pub struct Console {
    state: State
}

impl Console {
    pub fn new(state: State) -> Self {
        Console { state }
    }
}

impl AppContext for Console {
    fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        ui.child_frame(im_str!("logs"), (340.0, 172.0))
            .show_scrollbar(true)
            .show_borders(true)
            .build(|| {
                self.state.logs().iter().for_each(|log| ui.text(log));
            });
        true
    }
}

