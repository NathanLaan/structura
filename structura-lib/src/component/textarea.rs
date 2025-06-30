//!
//!
//!

use crate::component::Component;
use crate::event::MouseInput;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;
use softbuffer::Buffer;

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

    fn is_inside(&self, x: f64, y: f64) -> bool {
        let px = self.position.x;
        let py = self.position.y;
        let sw = self.size.width as f64;
        let sh = self.size.height as f64;
        x >= px && x <= px + sw && y >= py && y <= py + sh
    }
}

impl Component for TextArea {
    fn update(&mut self, input: MouseInput) {
        match input {
            // MouseInput::Click { x, y } => {
            //     self.focused = self.is_inside(x, y);
            // }
            // MouseInput::TextInput(s) => {
            //     if self.focused {
            //         self.text.insert_str(self.cursor_index, &s);
            //         self.cursor_index += s.len();
            //     }
            // }
            // MouseInput::KeyPress(c) => {
            //     if self.focused {
            //         match c {
            //             '\u{8}' => { // backspace
            //                 if self.cursor_index > 0 {
            //                     self.text.remove(self.cursor_index - 1);
            //                     self.cursor_index -= 1;
            //                 }
            //             }
            //             '\n' => {
            //                 self.text.insert(self.cursor_index, '\n');
            //                 self.cursor_index += 1;
            //             }
            //             _ => {
            //                 self.text.insert(self.cursor_index, c);
            //                 self.cursor_index += 1;
            //             }
            //         }
            //     }
            // }
            _ => {}
        }
    }

    fn draw(&self, context: &mut BufferContext) {
        let px = self.position.x as usize;
        let py = self.position.y as usize;
        let w = self.size.width;
        let h = self.size.height;
        let screen_w = context.screen_size.width as usize;
        let screen_h = context.screen_size.height as usize;

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
        for (i, _) in self.text.chars().enumerate() {
            let tx = px + i * 6;
            let ty = py + 10;
            if tx < screen_w && ty < screen_h {
                let offset = ty * screen_w + tx;
                context.buffer[offset] = 0xFF000000;
            }
        }

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
