use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use serde_json;
use imgui::*;

#[derive(Serialize, Deserialize)]
struct AppData {
    lyrics: String,
    timings: Vec<TimeFrame>,
    path: String
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TimeFrame {
    pub start: f32,
    pub end: f32,
    pub tooltip: String,
    #[serde(skip)]
    pub remove: bool
}

impl TimeFrame {
    pub fn new<T: ToString>(tooltip: T) -> Self {
        TimeFrame {
            tooltip: tooltip.to_string(),
            .. Default::default() 
        }
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
    pub fn new() -> Self {
        State(Rc::new(RefCell::new(InnerState {
            lyrics: ImString::with_capacity(10000),
            timings: Vec::new(),
            path: ImString::with_capacity(256),
            logs: Vec::new(),
        })))
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) {
        use std::io::Write;

        let mut file = File::create(path).expect("Could not create file");
        file.write(serde_json::to_string(&self.to_app_data()).unwrap().as_bytes()).unwrap();
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> bool {
        match load(path) {
            Ok(data) => {
                self.update_from_app_data(data);
                true
            }
            Err(_) => false
        }
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

    fn to_app_data(&self) -> AppData {
        let this = self.0.borrow();
        AppData {
            lyrics: this.lyrics.to_str().to_owned(),
            timings: this.timings.iter().cloned().collect(),
            path: this.path.to_str().to_owned()
        }
    }

    fn update_from_app_data(&self, saved_state: AppData) {
        let mut this = self.0.borrow_mut();
        this.lyrics = ImString::with_capacity(10000);
        this.lyrics.push_str(&saved_state.lyrics);
        this.path = ImString::with_capacity(256);
        this.path.push_str(&saved_state.path);
        this.timings = saved_state.timings.into_iter().collect();
    }
}

fn load<P: AsRef<Path>>(path: P) -> Result<AppData, ()> {
    use std::io::Read;

    let mut file = File::open(path.as_ref()).map_err(|_| ())?;
    let mut json = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut json).map_err(|_| ())?;
    serde_json::from_str::<AppData>(&json).map_err(|_| ())
}

