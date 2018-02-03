use imgui::*;

use dialogs::AppData;

pub struct State {
    lyrics: ImString,
    timings: Vec<([f32; 2], bool)>,
    path: ImString,
}

impl State {
    pub fn new() -> Self {
        State {
            lyrics: ImString::with_capacity(1000),
            timings: Vec::new(),
            path: ImString::with_capacity(256),
        }
    }

    pub fn to_app_data(&self) -> AppData {
        AppData {
            lyrics: self.lyrics.to_str().to_owned(),
            timings: self.timings.iter().map(|&(x, _)| (x[0], x[1])).collect(),
            path: self.path.to_str().to_owned()
        }
    }

    pub fn update_from_app_data(&mut self, mut saved_state: AppData) {
        self.lyrics = ImString::with_capacity(10000);
        self.lyrics.push_str(&saved_state.lyrics);
        self.path = ImString::with_capacity(256);
        self.path.push_str(&saved_state.path);
        self.timings = saved_state.timings.drain(..).map(|(x, y)| ([x, y], false)).collect();
    }

    pub fn lyrics_mut(&mut self) -> &mut ImString {
        &mut self.lyrics
    }

    pub fn timings(&self) -> &[([f32; 2], bool)] {
        self.timings.as_slice()
    }

    pub fn timings_mut(&mut self) -> &mut Vec<([f32; 2], bool)> {
        &mut self.timings
    }

    pub fn clean_timings(&mut self) {
        self.timings.clear();
    }

    pub fn path(&self) -> &ImString {
        &self.path
    }

    pub fn path_mut(&mut self) -> &mut ImString {
        &mut self.path
    }
}
