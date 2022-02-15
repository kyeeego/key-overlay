use device_query::Keycode;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

use crate::config::Config;

pub struct Key {
    is_pressed: bool,
    changed: bool,
    bars: Vec<Rect>,

    texture: Texture,

    container: Rect,

    config: Config,

    keycode: Keycode,
}

impl Key {
    pub fn new(index: u32, config: Config, keycode: Keycode, texture: Texture) -> Self {
        let container = Rect::new(
            (config.key.size / 2 + config.key.size / 2 * index + config.key.size * index) as i32,
            config.window.height as i32 - (config.key.size as f64 * 1.5).round() as i32,
            config.key.size,
            config.key.size,
        );

        Self {
            is_pressed: false,
            changed: false,
            bars: Vec::new(),

            texture,

            container,

            config,

            keycode,
        }
    }

    fn update_rects_pos(&mut self, delta: f64, speed: f64) {
        for bar in &mut self.bars {
            bar.y -= (speed * delta).round() as i32;
        }
    }

    pub fn remove_invisible_rect(&mut self) {
        if let Some(bar) = self.bars.first() {
            if bar.y + bar.h <= 0 {
                self.bars.remove(0);
            }
        }
    }

    pub fn update_rects(&mut self, delta: f64, speed: f64) {
        self.update_rects_pos(delta, speed);

        if self.changed && self.is_pressed {
            self.bars.push(Rect::new(
                self.container.x,
                self.container.y,
                self.config.key.size,
                0,
            ));
        } else if !self.changed && self.is_pressed {
            let r = self.bars.last_mut().unwrap();
            r.h += (speed * delta).round() as i32;
        }
    }

    pub fn update_state(&mut self, state: &Vec<Keycode>) {
        let prev_state = self.is_pressed;
        self.is_pressed = state.contains(&self.keycode);
        self.changed = prev_state != self.is_pressed;
    }

    fn draw_code(&self, canvas: &mut Canvas<Window>) {
        let w = self.config.key.size / 5;
        let h = self.config.key.size / 3;
        let r = Rect::new(
            self.container.x + self.config.key.size as i32 / 2 - w as i32 / 2,
            self.container.y + self.config.key.size as i32 / 2 - h as i32 / 2,
            w,
            h,
        );
        canvas.copy(&self.texture, None, r);
    }

    fn draw_key(&self, canvas: &mut Canvas<Window>, color: Color) {
        if self.is_pressed {
            let r = Rect::new(
                self.container.x,
                self.container.y,
                self.config.key.size,
                self.config.key.size,
            );

            canvas.set_draw_color(color);
            canvas.fill_rect(r);
        }

        for i in 0..self.config.key.border_width as i32 {
            let r = Rect::new(
                self.container.x + i,
                self.container.y + i + 1,
                self.config.key.size - i as u32 * 2,
                self.config.key.size - i as u32 * 2,
            );

            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_rect(r);
        }

        self.draw_code(canvas);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        self.draw_key(canvas, color);

        for bar in &self.bars {
            canvas.set_draw_color(color);
            canvas.draw_rect(bar.clone());
            canvas.fill_rect(bar.clone());
        }
    }
}
