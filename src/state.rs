use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use imgui::*;

#[derive(Serialize, Deserialize)]
pub struct AppData {
    pub lyrics: String,
    pub timings: Vec<(f32, f32)>,
    pub path: String
}

#[derive(Default, Copy, Clone)]
pub struct TimeFrame {
    pub start: f32,
    pub end: f32,
    pub remove: bool
}

impl TimeFrame {
    fn new(start: f32, end: f32) -> Self {
        TimeFrame { start, end, remove: false }
    }
}

struct InnerState {
    lyrics: ImString,
    timings: Vec<TimeFrame>,
    path: ImString,
    logs: Vec<String>,
}

#[derive(Clone)]
pub struct State(Rc<RefCell<InnerState>>);

impl State {
    #[inline]
    pub fn new() -> Self {
        State(Rc::new(RefCell::new(InnerState {
            lyrics: ImString::with_capacity(10000),
            timings: Vec::new(),
            path: ImString::with_capacity(256),
            logs: Vec::new(),
        })))
    }

    #[inline]
    pub fn to_app_data(&self) -> AppData {
        let this = self.0.borrow();
        AppData {
            lyrics: this.lyrics.to_str().to_owned(),
            timings: this.timings.iter().map(|&x| (x.start, x.end)).collect(),
            path: this.path.to_str().to_owned()
        }
    }

    #[inline]
    pub fn update_from_app_data(&self, saved_state: AppData) {
        let mut this = self.0.borrow_mut();
        this.lyrics = ImString::with_capacity(10000);
        this.lyrics.push_str(&saved_state.lyrics);
        this.path = ImString::with_capacity(256);
        this.path.push_str(&saved_state.path);
        this.timings = saved_state.timings
            .into_iter()
            .map(|(x, y)| TimeFrame::new(x, y))
            .collect();
    }

    #[inline]
    pub fn lyrics_mut<'a>(&'a mut self) -> RefMut<'a, ImString> {
        RefMut::map(self.0.borrow_mut(), |x| &mut x.lyrics)
    }

    #[inline]
    pub fn timings_mut<'a>(&'a mut self) -> RefMut<'a, Vec<TimeFrame>> {
        RefMut::map(self.0.borrow_mut(), |x| &mut x.timings)
    }

    #[inline]
    pub fn path<'a>(&'a self) -> Ref<'a, ImString> {
        Ref::map(self.0.borrow(), |x| &x.path)
    }

    #[inline]
    pub fn path_mut<'a>(&'a mut self) -> RefMut<'a, ImString> {
        RefMut::map(self.0.borrow_mut(), |x| &mut x.path)
    }

    #[inline]
    pub fn logs<'a>(&'a self) -> Ref<'a, [String]> {
        Ref::map(self.0.borrow(), |x| x.logs.as_slice())
    }

    #[inline]
    pub fn log(&mut self, log: String) {
        self.0.borrow_mut().logs.push(log);
    }
}
