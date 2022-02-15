use std::{str::FromStr, time::Instant};

use device_query::{DeviceQuery, DeviceState, Keycode};
use sdl2::{event::Event, pixels::Color, render::Canvas, video::Window, EventPump};

use crate::{config::Config, key::Key};

pub struct App {
    keys: Vec<Key>,

    state: DeviceState,

    sdl_event_pump: EventPump,
    canvas: Canvas<Window>,

    config: Config,
}

impl App {
    pub fn new(config: Config) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let font_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let spacing = config.key.size / 2;

        let screen_width = config.key.size * config.key.codes.len() as u32
            + spacing * (config.key.codes.len() as u32 + 1);

        let window = video_subsystem
            .window("Key Overlay", screen_width, config.window.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;
        let tc = canvas.texture_creator();

        let event_pump = sdl_context.event_pump()?;

        let font = font_context.load_font(config.clone().window.font_path, 256)?;

        let mut keys: Vec<Key> = Vec::new();

        for (i, k) in config.key.codes.iter().enumerate() {
            let key_surface = font
                .render(&*k)
                .blended(Color::WHITE)
                .map_err(|e| e.to_string())?;
            let key_texture = tc
                .create_texture_from_surface(key_surface)
                .map_err(|e| e.to_string())?;

            let kc = Keycode::from_str(&*k)?;

            keys.push(Key::new(i as u32, config.clone(), kc, key_texture));
        }

        Ok(Self {
            keys,

            state: DeviceState::new(),

            canvas,
            sdl_event_pump: event_pump,

            config,
        })
    }

    pub fn run(&mut self) {
        let mut delta = 0.;
        'running: loop {
            let frame_start_time = Instant::now();
            for event in self.sdl_event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            self.update(delta);
            self.draw();

            delta = frame_start_time.elapsed().as_secs_f64();
        }
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        for key in &self.keys {
            key.draw(&mut self.canvas, Color::RGB(100, 100, 100))
        }

        self.canvas.present();
    }

    fn update(&mut self, delta: f64) {
        let state = self.state.get_keys();
        for key in &mut self.keys {
            key.update_state(&state);
            key.remove_invisible_rect();
            key.update_rects(delta, self.config.scroll_speed);
        }
    }
}
