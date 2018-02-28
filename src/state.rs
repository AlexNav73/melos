
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use serde_json;
use imgui::*;

const DEFAULT_LYRICS_TEXT_SIZE: usize = 10000;
const SAVE_FILE_EXT: &str = "json";

#[derive(Serialize, Deserialize)]
struct AppData {
    lyrics: Vec<LanguageTab>,
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

#[derive(Serialize, Deserialize)]
struct LanguageTab {
    lang: String,
    text: String
}

pub struct ImLanguageTab {
    pub lang: ImString,
    pub text: ImString
}

impl<'a> From<&'a ImLanguageTab> for LanguageTab {
    fn from(tab: &'a ImLanguageTab) -> Self {
        LanguageTab {
            lang: tab.lang.to_str().to_owned(),
            text: tab.text.to_str().to_owned(),
        }
    }
}

impl From<LanguageTab> for ImLanguageTab {
    fn from(tab: LanguageTab) -> Self {
        let mut text = ImString::with_capacity(DEFAULT_LYRICS_TEXT_SIZE);
        text.push_str(&tab.text);
        ImLanguageTab {
            lang: ImString::new(tab.lang),
            text
        }
    }
}

impl ImLanguageTab {
    pub fn new<L, T>(lang: L, text: T) -> Self
        where L: AsRef<str>,
              T: AsRef<str>
    {
        let mut t = ImString::with_capacity(DEFAULT_LYRICS_TEXT_SIZE);
        t.push_str(text.as_ref());
        ImLanguageTab {
            lang: ImString::new(lang.as_ref()),
            text: t
        }
    }
}

impl Default for ImLanguageTab {
    fn default() -> Self {
        ImLanguageTab::new("en", "")
    }
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
    lyrics: Vec<ImLanguageTab>,
    timings: Vec<TimeFrame>,
    path: ImString,
    logs: Vec<String>,
}

#[derive(Clone)]
pub struct State(Rc<RefCell<InnerState>>);

impl State {
    pub fn new() -> Self {
        let mut lyrics = Vec::new();
        lyrics.push(ImLanguageTab::default());
        State(Rc::new(RefCell::new(InnerState {
            timings: Vec::new(),
            path: ImString::with_capacity(256),
            logs: Vec::new(),
            lyrics,
        })))
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) {
        use std::io::Write;

        let mut file = File::create(path.as_ref().with_extension(SAVE_FILE_EXT)).expect("Could not create file");
        file.write(serde_json::to_string(&self.to_app_data()).unwrap().as_bytes()).unwrap();
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> bool {
        load(path)
            .map(|data| { self.update(data); true })
            .map_err(|_| false)
            .unwrap()
    }

    #[inline]
    pub fn lyrics<'a>(&'a mut self) -> Ref<'a, Vec<ImLanguageTab>> {
        Ref::map(self.0.borrow(), |x| &x.lyrics)
    }

    #[inline]
    pub fn lyrics_mut<'a>(&'a mut self) -> RefMut<'a, Vec<ImLanguageTab>> {
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
            lyrics: this.lyrics.iter().map(|t| t.into()).collect(),
            timings: this.timings.iter().cloned().collect(),
            path: this.path.to_str().to_owned()
        }
    }

    fn update(&self, saved_state: AppData) {
        let mut this = self.0.borrow_mut();
        this.lyrics = saved_state.lyrics.into_iter().map(|t| t.into()).collect();
        this.path = ImString::with_capacity(256);
        this.path.push_str(&saved_state.path);
        this.timings = saved_state.timings.into_iter().collect();
    }
}

fn load<P: AsRef<Path>>(path: P) -> Result<AppData, ()> {
    use std::io::Read;

    let mut file = File::open(path.as_ref().with_extension(SAVE_FILE_EXT)).map_err(|_| ())?;
    let mut json = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut json).map_err(|_| ())?;
    serde_json::from_str::<AppData>(&json).map_err(|_| ())
}
