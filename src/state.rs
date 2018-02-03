use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use imgui::*;

use dialogs::AppData;

struct InnerState {
    lyrics: ImString,
    timings: Vec<([f32; 2], bool)>,
    path: ImString,
}

#[derive(Clone)]
pub struct State(Rc<RefCell<InnerState>>);

impl State {
    pub fn new() -> Self {
        State(Rc::new(RefCell::new(InnerState {
            lyrics: ImString::with_capacity(1000),
            timings: Vec::new(),
            path: ImString::with_capacity(256),
        })))
    }

    pub fn to_app_data(&self) -> AppData {
        let this = self.0.borrow();
        AppData {
            lyrics: this.lyrics.to_str().to_owned(),
            timings: this.timings.iter().map(|&(x, _)| (x[0], x[1])).collect(),
            path: this.path.to_str().to_owned()
        }
    }

    pub fn update_from_app_data(&self, mut saved_state: AppData) {
        let mut this = self.0.borrow_mut();
        this.lyrics = ImString::with_capacity(10000);
        this.lyrics.push_str(&saved_state.lyrics);
        this.path = ImString::with_capacity(256);
        this.path.push_str(&saved_state.path);
        this.timings = saved_state.timings.drain(..).map(|(x, y)| ([x, y], false)).collect();
    }

    pub fn lyrics_mut<'a>(&'a mut self) -> RefMut<'a, ImString> {
        RefMut::map(self.0.borrow_mut(), |x| &mut x.lyrics)
    }

    pub fn timings_mut<'a>(&'a mut self) -> RefMut<'a, Vec<([f32; 2], bool)>> {
        RefMut::map(self.0.borrow_mut(), |x| &mut x.timings)
    }

    pub fn clean_timings(&self) {
        self.0.borrow_mut().timings.clear();
    }

    pub fn path<'a>(&'a self) -> Ref<'a, ImString> {
        Ref::map(self.0.borrow(), |x| &x.path)
    }

    pub fn path_mut<'a>(&'a mut self) -> RefMut<'a, ImString> {
        RefMut::map(self.0.borrow_mut(), |x| &mut x.path)
    }
}
