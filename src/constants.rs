
pub mod global {
    pub const MAX_PATH_LEN: usize = 256;
}

pub mod main_window {
    pub const TOOLTIP_LEN: usize = 15;
    pub const LANG_NAME_LEN: usize = 5;
    pub const MAIN_WINDOW_SIZE: (f32, f32) = (620.0, 565.0);
    pub const LYRICS_INPUT_WIDTH: f32 = 550.0;
    pub const LYRICS_INPUT_HEIGHT: f32 = 530.0;
    pub const COLUMN_OFFSET: f32 = 560.0;
    pub const SONG_PATH_INPUT_LEN: f32 = 300.0;
    pub const TIMEFRAME_TOOLTIP_WIDTH: f32 = 100.0;
    pub const NEW_LANG_INPUT_WIDTH: f32 = 40.0;
    pub const QUATRAINS_FRAME_SIZE: (f32, f32) = (340.0, 200.0);
}

pub mod console {
    pub const CONSOLE_SIZE: (f32, f32) = (340.0, 172.0);
}

pub mod player {
    pub const PLAYER_FRAME_SIZE: (f32, f32) = (340.0, 100.0);
    pub const DEFAULT_VOLUME: f32 = 50.0;
}

pub mod dialogs {
    pub const BASE_DIR: &str = ".";
    pub const SAVE_FILE_EXT_FILTER: &str = "*.json";
    pub const DIALOG_SIZES: (f32, f32) = (275.0, 165.0);
    pub const FILE_BROWSER_WIDTH: f32 = 260.0;
}

pub mod state {
    pub const DEFAULT_LYRICS_TEXT_SIZE: usize = 10000;
    pub const SAVE_FILE_EXT: &str = "json";
    pub const DEFAULT_TAB_LANG: &str = "en";
}

