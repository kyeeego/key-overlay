use device_query::Keycode;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

pub struct Key {
    is_pressed: bool,
    changed: bool,
    bars: Vec<Rect>,

    texture: Texture,

    rect: Rect,

    index: u32,
    size: u32,
    border_size: u32,
    space_around: u32,

    keycode: Keycode,
}

impl Key {
    pub fn new(
        index: u32,
        size: u32,
        border_size: u32,
        space_around: u32,
        keycode: Keycode,
        texture: Texture,
    ) -> Self {
        let rect = Rect::new(
            (space_around + space_around * index + size * index) as i32,
            801,
            size,
            size,
        );

        Self {
            is_pressed: false,
            changed: false,
            bars: Vec::new(),

            texture,

            rect,

            index,
            size,
            border_size,
            space_around,

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
            self.bars
                .push(Rect::new(self.rect.x, self.rect.y, self.size, 0));
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
        let w = self.size / 5;
        let h = self.size / 3;
        let r = Rect::new(
            self.rect.x + self.size as i32 / 2 - w as i32 / 2,
            self.rect.y + self.size as i32 / 2 - h as i32 / 2,
            w,
            h,
        );
        canvas.copy(&self.texture, None, r);
    }

    fn draw_key(&self, canvas: &mut Canvas<Window>, color: Color) {
        if self.is_pressed {
            let r = Rect::new(self.rect.x, self.rect.y, self.size, self.size);

            canvas.set_draw_color(color);
            canvas.fill_rect(r);
        }

        for i in 0..self.border_size as i32 {
            let r = Rect::new(
                self.rect.x + i,
                self.rect.y + i + 1,
                self.size - i as u32 * 2,
                self.size - i as u32 * 2,
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
