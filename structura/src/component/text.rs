//!
//!
//!

use winit::event::{KeyEvent, MouseScrollDelta, TouchPhase};
use crate::component::Component;
use crate::event::MouseInput;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;

pub struct Text {
    pub content: String,
    pub x: i32,
    pub y: i32,
}

pub struct GapBuffer {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    ///
    /// Constructor.
    ///
    pub fn new(size: usize) -> Self {
        Self {
            buffer: vec!['\0'; size],
            gap_start: 0,
            gap_end: size,
        }
    }

    ///
    /// Insert a character at the current cursor position.
    ///
    pub fn insert(&mut self, c: char) {
        if self.gap_start == self.gap_end {
            self.grow();
        }
        self.buffer[self.gap_start] = c;
        self.gap_start += 1;
    }

    ///
    /// Move the cursor left.
    ///
    pub fn move_left(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.gap_end -= 1;
            self.buffer[self.gap_end] = self.buffer[self.gap_start];
        }
    }

    ///
    /// Move the cursor right.
    ///
    pub fn move_right(&mut self) {
        if self.gap_end < self.buffer.len() {
            self.buffer[self.gap_start] = self.buffer[self.gap_end];
            self.gap_start += 1;
            self.gap_end += 1;
        }
    }

    ///
    /// Delete a character before the cursor.
    ///
    pub fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
        }
    }

    ///
    /// Grow the gap.
    ///
    fn grow(&mut self) {
        let new_capacity = self.buffer.len() * 2;
        let mut new_buffer = vec!['\0'; new_capacity];
        let gap_size = self.gap_end - self.gap_start;

        // Copy before gap
        new_buffer[..self.gap_start].copy_from_slice(&self.buffer[..self.gap_start]);

        // Copy after gap
        let after_gap_len = self.buffer.len() - self.gap_end;
        let new_gap_end = new_capacity - after_gap_len;
        new_buffer[new_gap_end..].copy_from_slice(&self.buffer[self.gap_end..]);

        self.buffer = new_buffer;
        self.gap_end = new_gap_end;
    }

    ///
    /// Get the contents as a String.
    ///
    pub fn contents(&self) -> String {
        self.buffer[..self.gap_start]
            .iter()
            .chain(&self.buffer[self.gap_end..])
            .collect()
    }

    ///
    /// Get current cursor position.
    ///
    pub fn cursor(&self) -> usize {
        self.gap_start
    }
}

pub struct TextEditor {
    pub buffer: GapBuffer,
}

impl TextEditor {
    ///
    /// Constructor
    ///
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: GapBuffer::new(capacity),
        }
    }
    
    fn draw_text(&mut self, buffer: &mut [u32], width: u32, text: &str) {
        for (i, c) in text.chars().enumerate() {
            let x = (i as u32 % (width / 8)) * 8;
            let y = (i as u32 / (width / 8)) * 16;
            self.draw_char(buffer, width, x, y, c);
        }
    }

    ///
    /// Draw character
    /// 
    fn draw_char(&mut self, buffer: &mut [u32], width: u32, x: u32, y: u32, _c: char) {
        for dy in 0..16 {
            for dx in 0..8 {
                let px = x + dx;
                let py = y + dy;
                if px < width {
                    let idx = (py * width + px) as usize;
                    if idx < buffer.len() {
                        buffer[idx] = 0x000000; // black pixels (placeholder)
                    }
                }
            }
        }
    }

}

impl Component for TextEditor {
    fn handle_mouse_event(&mut self, input: MouseInput) {
        todo!()
    }

    fn handle_mouse_wheel_event(&mut self, event: &MouseScrollDelta, phase: &TouchPhase) {
        todo!()
    }

    fn handle_keyboard_event(&mut self, event: &KeyEvent) {
        todo!()
    }

    fn draw(&self, context: &mut BufferContext) {
        // for pixel in context.buffer {
        //     *pixel = 0xFFFFFF; // white background
        // }
        //
        // // Naive monospaced rendering (each char = 8x16 pixel block)
        // draw_text(&mut frame, width, &buffer.contents());
    }

    fn set_position(&mut self, x: f64, y: f64) {
        todo!()
    }

    fn get_position(&self) -> Point {
        todo!()
    }

    fn set_size(&mut self, width: usize, height: usize) {
        todo!()
    }

    fn get_size(&self) -> Size {
        todo!()
    }
}
