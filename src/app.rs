use std::{io::Stdin, time::Instant};

use device_query::{DeviceQuery, DeviceState, Keycode};
use sdl2::{
    event::Event, pixels::Color, render::Canvas, video::Window, EventPump, IntegerOrSdlError,
};

use crate::key::Key;

static SPACE_BETWEEN: u32 = 80;
static KEY_SIZE: u32 = 130;

static WIDTH: u32 = 500;
static HEIGHT: u32 = 1000;

pub struct App {
    keys: Vec<Key>,

    state: DeviceState,

    sdl_event_pump: EventPump,
    canvas: Canvas<Window>,

    speed: f64,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let font_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let window = video_subsystem
            .window("Key Overlay", WIDTH, HEIGHT)
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

        let mut font = font_context.load_font("Montserrat-Bold.ttf", 128)?;

        let h_surface = font.render("H").blended(Color::WHITE).unwrap();
        let k_surface = font.render("K").blended(Color::WHITE).unwrap();

        let h_texture = tc.create_texture_from_surface(h_surface).unwrap();
        let k_texture = tc.create_texture_from_surface(k_surface).unwrap();

        let keys = vec![
            Key::new(0, KEY_SIZE, 8, SPACE_BETWEEN, Keycode::H, h_texture),
            Key::new(1, KEY_SIZE, 8, SPACE_BETWEEN, Keycode::K, k_texture),
        ];

        Ok(Self {
            keys,

            state: DeviceState::new(),

            canvas,
            sdl_event_pump: event_pump,

            speed: 1000.,
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
            key.update_rects(delta, self.speed);
        }
    }
}
