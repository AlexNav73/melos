
use config::{Config as Config_, File};

use constants::*;

#[derive(Debug)]
pub struct Config {
    pub state: State,
    pub dialogs: Dialogs,
    pub player: Player,
    pub console: Console,
    pub main_window: MainWindow,
}

#[derive(Debug)]
pub struct State {
    pub default_lyrics_text_size: usize,
    pub save_file_ext: String,
    pub default_tab_lang: String,
}

#[derive(Debug)]
pub struct Dialogs {
    pub base_dir: String,
    pub save_file_ext_filter: String,
    pub dialog_sizes: (f32, f32),
    pub file_browser_width: f32,
}

#[derive(Debug)]
pub struct Player {
    pub player_frame_size: (f32, f32),
    pub default_volume: f32,
}

#[derive(Debug)]
pub struct Console {
    pub console_size: (f32, f32),
}

#[derive(Debug)]
pub struct MainWindow {
    pub tooltip_len: usize,
    pub lang_name_len: usize,
    pub main_window_size: (f32, f32),
    pub lyrics_input_width: f32,
    pub lyrics_input_height: f32,
    pub column_offset: f32,
    pub song_path_input_len: f32,
    pub timeframe_tooltip_width: f32,
    pub new_lang_input_width: f32,
    pub quatrains_frame_size: (f32, f32),
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let mut config = Config_::new();
        config.merge(File::with_name("Settings.toml").required(false)).unwrap();

        let map: Settings = config.try_into().unwrap();
        let state = map.state.unwrap_or(State_::default());
        let dialogs = map.dialogs.unwrap_or(Dialogs_::default());
        let player = map.player.unwrap_or(Player_::default());
        let console = map.console.unwrap_or(Console_::default());
        let main_window = map.main_window.unwrap_or(MainWindow_::default());

        Config {
            state: State {
                default_lyrics_text_size: state.default_lyrics_text_size.unwrap_or(state::DEFAULT_LYRICS_TEXT_SIZE),
                save_file_ext: state.save_file_ext.unwrap_or(state::SAVE_FILE_EXT.into()),
                default_tab_lang: state.default_tab_lang.unwrap_or(state::DEFAULT_TAB_LANG.into())
            },
            dialogs: Dialogs {
                base_dir: dialogs.base_dir.unwrap_or(dialogs::BASE_DIR.into()),
                save_file_ext_filter: dialogs.save_file_ext_filter.unwrap_or(dialogs::SAVE_FILE_EXT_FILTER.into()),
                dialog_sizes: dialogs.dialog_sizes.unwrap_or(dialogs::DIALOG_SIZES),
                file_browser_width: dialogs.file_browser_width.unwrap_or(dialogs::FILE_BROWSER_WIDTH),
            },
            player: Player {
                player_frame_size: player.player_frame_size.unwrap_or(player::PLAYER_FRAME_SIZE),
                default_volume: player.default_volume.unwrap_or(player::DEFAULT_VOLUME)
            },
            console: Console {
                console_size: console.console_size.unwrap_or(console::CONSOLE_SIZE),
            },
            main_window: MainWindow {
                tooltip_len: main_window.tooltip_len.unwrap_or(main_window::TOOLTIP_LEN),
                lang_name_len: main_window.lang_name_len.unwrap_or(main_window::LANG_NAME_LEN),
                main_window_size: main_window.main_window_size.unwrap_or(main_window::MAIN_WINDOW_SIZE),
                lyrics_input_width: main_window.lyrics_input_width.unwrap_or(main_window::LYRICS_INPUT_WIDTH),
                lyrics_input_height: main_window.lyrics_input_height.unwrap_or(main_window::LYRICS_INPUT_HEIGHT),
                column_offset: main_window.column_offset.unwrap_or(main_window::COLUMN_OFFSET),
                song_path_input_len: main_window.song_path_input_len.unwrap_or(main_window::SONG_PATH_INPUT_LEN),
                timeframe_tooltip_width: main_window.timeframe_tooltip_width.unwrap_or(main_window::TIMEFRAME_TOOLTIP_WIDTH),
                new_lang_input_width: main_window.new_lang_input_width.unwrap_or(main_window::NEW_LANG_INPUT_WIDTH),
                quatrains_frame_size: main_window.quatrains_frame_size.unwrap_or(main_window::QUATRAINS_FRAME_SIZE),
            }
        }
    };
}

#[derive(Debug, Deserialize)]
struct Settings {
    state: Option<State_>,
    dialogs: Option<Dialogs_>,
    player: Option<Player_>,
    console: Option<Console_>, main_window: Option<MainWindow_>
}

#[derive(Debug, Default, Deserialize)]
struct State_ {
    default_lyrics_text_size: Option<usize>,
    save_file_ext: Option<String>,
    default_tab_lang: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct Dialogs_ {
    base_dir: Option<String>,
    save_file_ext_filter: Option<String>,
    dialog_sizes: Option<(f32, f32)>,
    file_browser_width: Option<f32>,
}

#[derive(Debug, Default, Deserialize)]
struct Player_ {
    player_frame_size: Option<(f32, f32)>,
    default_volume: Option<f32>,
}

#[derive(Debug, Default, Deserialize)]
struct Console_ {
    console_size: Option<(f32, f32)>,
}

#[derive(Debug, Default, Deserialize)]
struct MainWindow_ {
    tooltip_len: Option<usize>,
    lang_name_len: Option<usize>,
    main_window_size: Option<(f32, f32)>,
    lyrics_input_width: Option<f32>,
    lyrics_input_height: Option<f32>,
    column_offset: Option<f32>,
    song_path_input_len: Option<f32>,
    timeframe_tooltip_width: Option<f32>,
    new_lang_input_width: Option<f32>,
    quatrains_frame_size: Option<(f32, f32)>,
}
