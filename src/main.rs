use std::fs::File;

use app::App;
use config::Config;

mod app;
mod config;
mod key;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let conf_path = match args.get(1) {
        Some(p) => p.clone(),
        None => "config.yml".to_string(),
    };

    let mut config = Config::default();

    match File::open(conf_path) {
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

    match App::new(config) {
        Ok(mut app) => app.run(),
        Err(e) => Err(format!("Unable to start the app: {}", e)),
    }
}
