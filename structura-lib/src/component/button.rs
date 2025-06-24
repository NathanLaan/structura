//!
//! Rust MVC-UI
//!
use crate::event::Callback;
use rusttype::{Font, Scale, point};

pub struct Button {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub color: u32,
    pub label: &'static str,
    pub on_clicked: Option<Callback<()>>,
}

impl Button {
    pub fn contains(&self, px: usize, py: usize) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }

    pub fn draw(&self, buffer: &mut [u32], screen_width: usize, font: &Font<'_>) {
        for y in self.y..(self.y + self.height) {
            for x in self.x..(self.x + self.width) {
                let idx = y * screen_width + x;
                buffer[idx] = self.color;
            }
        }

        // Text rendering parameters
        let font_scale = Scale::uniform(20.0);
        let v_metrics = font.v_metrics(font_scale);

        let start_x = self.x as i32 + 10;
        let start_y = self.y as i32 + (self.height as i32 / 2) + (v_metrics.ascent / 2.0) as i32;

        let glyphs: Vec<_> = font
            .layout(
                self.label,
                font_scale,
                point(start_x as f32, start_y as f32),
            )
            .collect();

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let x = gx as i32 + bb.min.x;
                    let y = gy as i32 + bb.min.y;
                    if x >= 0
                        && x < screen_width as i32
                        && y >= 0
                        && (y as usize) < buffer.len() / screen_width
                    {
                        let idx = y as usize * screen_width + x as usize;
                        let value = (v * 255.0) as u8;
                        buffer[idx] = Self::blend_pixel(buffer[idx], value);
                    }
                });
            }
        }
    }

    fn blend_pixel(bg: u32, brightness: u8) -> u32 {
        let r = brightness as u32;
        let g = brightness as u32;
        let b = brightness as u32;
        (r << 16) | (g << 8) | b
    }
}
