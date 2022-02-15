use std::fs::File;

use app::App;
use config::Config;

mod app;
mod config;
mod key;

fn main() {
    let conf_f = match File::open("example.config.yml") {
        Ok(f) => f,
        Err(e) => {
            println!("Unable to read config: {}", e.to_string());
            return;
        }
    };

    let config: Config = match serde_yaml::from_reader(conf_f) {
        Ok(c) => c,
        Err(e) => {
            println!("Unable to parse config: {}", e.to_string());
            return;
        }
    };

    match App::new(config) {
        Ok(mut app) => app.run(),
        Err(e) => println!("Unable to start the app: {}", e),
    }
}
