
use imgui::*;

use std::borrow::Borrow;

use song::TimeSpan;
use configuration::CONFIG;

#[derive(Serialize, Deserialize)]
pub struct AppData {
    pub lyrics: Vec<LanguageTab>,
    pub timings: Vec<TimeFrame>,
    pub path: String
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TimeFrame {
    pub start: f32,
    pub end: f32,
    pub tooltip: Option<String>,
    #[serde(skip)]
    pub remove: bool
}

impl<T: Borrow<TimeFrame>> From<T> for TimeSpan {
    fn from(value: T) -> TimeSpan {
        fn to_s(time: f32) -> u32 {
            let decimal = time as u32;
            let real = ((time - decimal as f32) * 100.0) as u32;
            decimal * 60 + real
        }

        let value = value.borrow();
        let start = to_s(value.start);
        let duration = to_s(value.end) - start;

        TimeSpan::new(start, duration)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LanguageTab {
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
        let mut text = ImString::with_capacity(CONFIG.state.default_lyrics_text_size);
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
        let mut t = ImString::with_capacity(CONFIG.state.default_lyrics_text_size);
        t.push_str(text.as_ref());
        ImLanguageTab {
            lang: ImString::new(lang.as_ref()),
            text: t
        }
    }
}

impl Default for ImLanguageTab {
    fn default() -> Self {
        ImLanguageTab::new(&CONFIG.state.default_tab_lang, "")
    }
}

impl TimeFrame {
    pub fn with_tooltip<T: ToString>(tooltip: T) -> Self {
        TimeFrame {
            tooltip: Some(tooltip.to_string()),
            .. Default::default() 
        }
    }
    pub fn new() -> Self {
        TimeFrame::default()
    }
}
