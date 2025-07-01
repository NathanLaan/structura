//!
//!
//!

use rusttype::{point, Scale};
use crate::component::Component;
use crate::event::{KeyboardInput, MouseInput};
use crate::geometry::{Point, Size};
use crate::view::BufferContext;
use softbuffer::Buffer;
use winit::keyboard::Key;

pub struct TextArea {
    pub text: String,
    pub cursor_index: usize,
    pub position: Point,
    pub size: Size,
    pub focused: bool,
}

impl TextArea {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor_index: 0,
            position: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 200,
                height: 50,
            },
            focused: false,
        }
    }

    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.position.x
            && px < self.position.x + self.size.width as f64
            && py >= self.position.y
            && py < self.position.y + self.size.height as f64
    }

    fn draw_text(&self, context: &mut BufferContext) {
        let font_scale = Scale::uniform(context.font_size);
        let v_metrics = context.font.v_metrics(font_scale);

        let screen_width = context.screen_size.width as usize;

        let start_x = self.position.x + 10.0;
        let start_y =
            self.position.y + (self.size.height as f64 / 2.0) + (v_metrics.ascent / 2.0) as f64;

        //
        // TODO: Move to text rendering component
        //
        let glyphs: Vec<_> = context
            .font
            .layout(
                &self.text[..],
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
                        && x < context.screen_size.width as i32
                        && y >= 0
                        && (y as usize) < context.buffer.len() / screen_width
                    {
                        let idx = y as usize * screen_width + x as usize;
                        context.buffer[idx] = Self::basic_aa(context.buffer[idx], 0x000000, v);
                    }
                });
            }
        }
    }

    fn basic_aa(bg: u32, fg: u32, alpha: f32) -> u32 {
        let inv = 1.0 - alpha;

        let br = ((bg >> 16) & 0xFF) as f32;
        let bg_ = ((bg >> 8) & 0xFF) as f32;
        let bb = (bg & 0xFF) as f32;

        let fr = ((fg >> 16) & 0xFF) as f32;
        let fg_ = ((fg >> 8) & 0xFF) as f32;
        let fb = (fg & 0xFF) as f32;

        let r = (br * inv + fr * alpha).round() as u32;
        let g = (bg_ * inv + fg_ * alpha).round() as u32;
        let b = (bb * inv + fb * alpha).round() as u32;

        (r << 16) | (g << 8) | b
    }
    
}

impl Component for TextArea {
    fn handle_mouse_event(&mut self, input: MouseInput) {
        if input.pressed {
            self.focused = self.contains(input.position.x, input.position.y);
        }
    }

    fn handle_keyboard_event(&mut self, event: &winit::event::KeyEvent) {
        if self.focused {
            match &event.logical_key {
                Key::Character(s) => {
                    self.text.push_str(s);
                    println!("Val: {:?}", self.text);
                }
                Key::Named(_) => {}
                Key::Unidentified(_) => {}
                Key::Dead(_) => {}
            }
        }
    }

    fn draw(&self, context: &mut BufferContext) {
        let px = self.position.x as usize;
        let py = self.position.y as usize;
        let w = self.size.width;
        let h = self.size.height;
        let screen_w = context.screen_size.width as usize;
        let screen_h = context.screen_size.height as usize;

        // Background
        for y in 0..h {
            for x in 0..w {
                let offset = (py + y as usize) * screen_w + (px + x as usize);
                if offset < context.buffer.len() {
                    context.buffer[offset] = if self.focused {
                        0xFFEEEEFF // background when focused
                    } else {
                        0xFFFFFFFF // background when not focused
                    };
                }
            }
        }

        //
        // TODO: Replace with font rendering
        //
        // for (i, _) in self.text.chars().enumerate() {
        //     let tx = px + i * 6;
        //     let ty = py + 10;
        //     if tx < screen_w && ty < screen_h {
        //         let offset = ty * screen_w + tx;
        //         context.buffer[offset] = 0xFF000000;
        //     }
        // }
        self.draw_text(context);

        // Draw cursor
        if self.focused {
            let cx = px + self.cursor_index * 6;
            let cy = py + 10;
            if cx < screen_w && cy < screen_h {
                context.buffer[cy * screen_w + cx] = 0xFFFF0000; // red cursor
            }
        }
    }
    

    fn set_position(&mut self, x: f64, y: f64) {
        self.position = Point { x, y };
    }

    fn get_position(&self) -> Point {
        self.position
    }

    fn set_size(&mut self, width: usize, height: usize) {
        self.size = Size {
            width: width as u32,
            height: height as u32,
        };
    }

    fn get_size(&self) -> Size {
        self.size.clone()
    }
}
