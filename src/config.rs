use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    #[serde(default = "WindowConf::default")]
    pub window: WindowConf,

    #[serde(default = "KeyConf::default")]
    pub key: KeyConf,

    #[serde(default = "def_scroll_speed")]
    pub scroll_speed: f64,
}

#[derive(Clone, Deserialize)]
pub struct WindowConf {
    #[serde(default = "def_height")]
    pub height: u32,

    #[serde(default = "def_font_path")]
    pub font_path: String,
}

#[derive(Clone, Deserialize)]
pub struct KeyConf {
    #[serde(default = "def_key_size")]
    pub size: u32,

    #[serde(default = "def_key_border_width")]
    pub border_width: u32,

    #[serde(default = "def_key_codes")]
    pub codes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConf::default(),
            key: KeyConf::default(),
            scroll_speed: def_scroll_speed(),
        }
    }
}

impl Default for KeyConf {
    fn default() -> Self {
        Self {
            size: def_key_size(),
            border_width: def_key_border_width(),
            codes: def_key_codes(),
        }
    }
}

impl Default for WindowConf {
    fn default() -> Self {
        Self {
            height: def_height(),
            font_path: def_font_path(),
        }
    }
}

fn def_scroll_speed() -> f64 {
    1000.
}

fn def_key_size() -> u32 {
    100
}

fn def_key_border_width() -> u32 {
    8
}

fn def_key_codes() -> Vec<String> {
    vec![String::from("Z"), String::from("X")]
}

fn def_height() -> u32 {
    800
}

fn def_font_path() -> String {
    String::from("Montserrat-Bold.ttf")
}
