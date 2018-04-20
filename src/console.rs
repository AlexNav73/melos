
use imgui::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub struct Console {
    logger: Logger,
    logs: Vec<String>
}

impl Console {
    pub fn new(mut logger: Logger) -> Self {
        logger.enable();
        Console {
            logger,
            logs: Vec::new() 
        }
    }

    pub fn show<'a>(&mut self, ui: &Ui<'a>) -> bool {
        const DISTANCE: f32 = 10.0;
        const CORNER: i32 = 0;

        let (display_size_x, display_size_y) = ui.imgui().display_size();
        let window_pos = (if CORNER & 1 > 0 { display_size_x - DISTANCE } else { DISTANCE }, if CORNER & 2 > 0 { display_size_y - DISTANCE } else { DISTANCE });
        let window_pos_pivot = (if CORNER & 1 > 0 { 1.0 } else { 0.0 }, if CORNER & 2 > 0 { 1.0 } else { 0.0 });

        if let Some(mut logs) = self.logger.full_logs() {
            self.logs.append(&mut logs);
        }

        let mut opened = true;
        ui.window(im_str!("##logs"))
            .position(window_pos, ImGuiCond::Always)
            .size((100.0, 100.0), ImGuiCond::Always)
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

impl Drop for Console {
    fn drop(&mut self) {
        self.logger.disable();
    }
}

#[derive(Clone)]
pub struct Logger(Rc<RefCell<Option<VecDeque<String>>>>);

impl Logger {
    pub fn new() -> Self {
        Logger(Rc::new(RefCell::new(None)))
    }

    pub fn enable(&mut self) {
        *self.0.borrow_mut() = Some(VecDeque::new());
    }

    pub fn disable(&mut self) {
        *self.0.borrow_mut() = None;
    }

    pub fn log<T: ToString>(&mut self, log: T) {
        if let Some(ref mut queue) = *self.0.borrow_mut() {
            queue.push_back(log.to_string());
        }
    }

    pub fn full_logs(&mut self) -> Option<Vec<String>> {
        self.0.borrow_mut().as_mut().map(|x| x.drain(..).collect())
    }
}
