//!
//! # Structura Component: TextArea.
//!
//! A toopltip that shows text and floats over another `Component`.
//!

use crate::component::Component;
use crate::event::MouseInput;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;

pub struct Tooltip {
    text: String,
    position: Point,
    size: Size,
    visible: bool,
}

impl Tooltip {
    pub fn new(text: String, position: Point, size: Size) -> Self {
        Self {
            text,
            position,
            size,
            visible: true,
        }
    }
}

impl Component for Tooltip {
    fn handle_mouse_event(&mut self, input: MouseInput) {}

    fn handle_mouse_wheel_event(
        &mut self,
        _event: &winit::event::MouseScrollDelta,
        _phase: &winit::event::TouchPhase,
    ) {
    }

    fn handle_keyboard_event(&mut self, _event: &winit::event::KeyEvent) {}

    fn draw(&self, context: &mut BufferContext) {
        if !self.visible {
            return;
        }

        //
        // TODO: Add utility functions to BufferContext
        //

        // Draw rectangle background
        //context.fill_rect(self.position.x, self.position.y, self.size.w, self.size.h, 0xFF000000);

        // Draw the tooltip text
        //context.draw_text(&self.text, self.position.x + 5.0, self.position.y + 5.0, 0xFFFFFFFF);
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
