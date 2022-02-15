use serde::Deserialize;

fn def_u32() -> u32 {
    1000
}

fn def_font_path() -> String {
    String::from("Montserrat-Bold.ttf")
}

#[derive(Clone, Deserialize)]
pub struct WindowConf {
    #[serde(default = "def_u32")]
    pub height: u32,

    #[serde(default = "def_font_path")]
    pub font_path: String,
}

fn def_key_size() -> u32 {
    130
}

fn def_key_border_width() -> u32 {
    8
}

fn def_key_codes() -> Vec<String> {
    vec![String::from("z"), String::from("x")]
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

fn def_scroll_speed() -> f64 {
    1000.
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub window: WindowConf,
    pub key: KeyConf,

    #[serde(default = "def_scroll_speed")]
    pub scroll_speed: f64,
}
