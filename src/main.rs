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

    let config = Config::new(conf_path)?;

    match App::new(config) {
        Ok(mut app) => app.run(),
        Err(e) => Err(format!("Unable to start the app: {}", e)),
    }
}
