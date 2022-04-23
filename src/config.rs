use std::fs::File;

use sdl2::pixels::Color;
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

    #[serde(skip_deserializing, default = "def_border_color")]
    pub border_color: Color,

    #[serde(skip_deserializing, default = "def_tile_color")]
    pub tile_color: Color,

    #[serde(rename = "border_color", default = "def_border_color_hex")]
    _border_color: String,

    #[serde(rename = "tile_color", default = "def_tile_color_hex")]
    _tile_color: String,
}

impl Config {
    pub fn new(path: String) -> Result<Self, String> {
        let mut config = Config::default();

        match File::open(path) {
            Ok(f) => {
                config = match serde_yaml::from_reader(f) {
                    Ok(c) => c,
                    Err(e) => {
                        return Err(format!("Unable to parse config: {}", e.to_string()));
                    }
                };
            }
            _ => {}
        };

        config.key.border_color =
            parse_hex(&*config.key._border_color).unwrap_or(def_border_color());
        config.key.tile_color = parse_hex(&*config.key._tile_color).unwrap_or(def_tile_color());

        Ok(config)
    }
}

fn parse_hex(s: &str) -> Result<Color, String> {
    if let Some(s) = s.strip_prefix('#') {
        let n = s.len();

        let (r, g, b, a) = if n == 6 || n == 8 {
            let r = u8::from_str_radix(&s[0..2], 16).map_err(|e| e.to_string())?;
            let g = u8::from_str_radix(&s[2..4], 16).map_err(|e| e.to_string())?;
            let b = u8::from_str_radix(&s[4..6], 16).map_err(|e| e.to_string())?;

            let a = if n == 8 {
                u8::from_str_radix(&s[6..8], 16).map_err(|e| e.to_string())?
            } else {
                255
            };

            (r, g, b, a)
        } else {
            return Err("Invalid color".to_string());
        };

        Ok(Color::RGBA(r, g, b, a))
    } else {
        Err("Invalid hex".to_string())
    }
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
            border_color: def_border_color(),
            _border_color: "#ffffff".to_string(),
            tile_color: def_tile_color(),
            _tile_color: "#646464".to_string(),
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

fn def_border_color() -> Color {
    Color::RGB(255, 255, 255)
}

fn def_border_color_hex() -> String {
    "#ffffff".to_string()
}

fn def_tile_color() -> Color {
    Color::RGB(100, 100, 100)
}

fn def_tile_color_hex() -> String {
    "#646464".to_string()
}
