use app::App;

mod app;
mod key;

fn main() {
    match App::new() {
        Ok(mut app) => app.run(),
        Err(e) => println!("Unable to start the app: {}", e.to_string()),
    }
}
