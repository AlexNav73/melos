
use config::{Config as Config_, File};

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
    pub default_tab_lang: String,
}

#[derive(Debug)]
pub struct Dialogs {
    pub base_dir: String,
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
    pub console_pos: (f32, f32),
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
        config.merge(File::with_name("Settings.toml").required(false))
            .expect("Can't merge settings file path");

        let map: Settings = config.try_into().expect("Can't parse settings file");
        let state = map.state.unwrap_or(State_::default());
        let dialogs = map.dialogs.unwrap_or(Dialogs_::default());
        let player = map.player.unwrap_or(Player_::default());
        let console = map.console.unwrap_or(Console_::default());
        let main_window = map.main_window.unwrap_or(MainWindow_::default());

        Config {
            state: State {
                default_lyrics_text_size: state.default_lyrics_text_size.unwrap_or(10000),
                default_tab_lang: state.default_tab_lang.unwrap_or("en".into())
            },
            dialogs: Dialogs {
                base_dir: dialogs.base_dir.unwrap_or(".".into()),
                dialog_sizes: dialogs.dialog_sizes.unwrap_or((275.0, 165.0)),
                file_browser_width: dialogs.file_browser_width.unwrap_or(260.0),
            },
            player: Player {
                player_frame_size: player.player_frame_size.unwrap_or((340.0, 105.0)),
                default_volume: player.default_volume.unwrap_or(50.0)
            },
            console: Console {
                console_pos: console.console_pos.unwrap_or((5.0, 25.0)),
            },
            main_window: MainWindow {
                main_window_size: main_window.main_window_size.unwrap_or((620.0, 565.0)),
                quatrains_frame_size: main_window.quatrains_frame_size.unwrap_or((340.0, 190.0)),
                tooltip_len: main_window.tooltip_len.unwrap_or(15),
                lang_name_len: main_window.lang_name_len.unwrap_or(5),
                lyrics_input_width: main_window.lyrics_input_width.unwrap_or(550.0),
                lyrics_input_height: main_window.lyrics_input_height.unwrap_or(530.0),
                column_offset: main_window.column_offset.unwrap_or(560.0),
                song_path_input_len: main_window.song_path_input_len.unwrap_or(300.0),
                timeframe_tooltip_width: main_window.timeframe_tooltip_width.unwrap_or(100.0),
                new_lang_input_width: main_window.new_lang_input_width.unwrap_or(40.0),
            }
        }
    };
}

#[derive(Debug, Deserialize)]
struct Settings {
    state: Option<State_>,
    dialogs: Option<Dialogs_>,
    player: Option<Player_>,
    console: Option<Console_>,
    main_window: Option<MainWindow_>
}

#[derive(Debug, Default, Deserialize)]
struct State_ {
    default_lyrics_text_size: Option<usize>,
    default_tab_lang: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct Dialogs_ {
    base_dir: Option<String>,
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
    console_pos: Option<(f32, f32)>,
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
