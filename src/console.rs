
use imgui::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

use configuration::*;

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
        if let Some(mut logs) = self.logger.full_logs() {
            self.logs.append(&mut logs);
        }

        let mut opened = true;
        ui.with_style_var(StyleVar::Alpha(0.3), || {
            ui.window(im_str!("##logs"))
                .position(CONFIG.console.console_pos, ImGuiCond::Always)
                .opened(&mut opened)
                .collapsible(false)
                .title_bar(false)
                .always_auto_resize(true)
                .no_focus_on_appearing(true)
                .build(|| {
                    ui.with_style_var(StyleVar::Alpha(1.0), || {
                        self.logs.iter().for_each(|log| ui.text(log));
                    });
                });
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
