//!
//! Structura Components.
//!

pub mod button;
pub mod text;
pub mod textarea;

use crate::event::{KeyboardInput, MouseInput};
use crate::geometry::Point;
use crate::geometry::Size;
use crate::view::BufferContext;
use rusttype::Font;

pub fn load_font() -> Font<'static> {
    let font_data = include_bytes!("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    font
}

#[derive(Debug, Clone)]
pub enum ComponentState {
    Active,
    Hovered,
    Pressed,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct ComponentStyle {
    pub text_color: u32,
    pub background_color: u32,
    pub border_color: u32,
}

impl Default for ComponentStyle {
    fn default() -> Self {
        Self {
            background_color: 0x0033CC,
            border_color: 0x000000,
            text_color: 0x000000,
        }
    }
}

///
/// Base trait for all UI components
///
pub trait Component {
    ///
    /// Called whenever the user makes an input (e.g. mouse moved, mouse pressed).
    ///
    fn handle_mouse_event(&mut self, input: MouseInput);

    fn handle_keyboard_event(&mut self, event: &winit::event::KeyEvent);

    ///
    /// Called whenever the application updates it's state, or a redraw is requested in response to an event.
    ///
    fn draw(&self, context: &mut BufferContext);

    ///
    /// Set the position of the `Component` relative to it's parent `Container`.
    ///
    fn set_position(&mut self, x: f64, y: f64);

    ///
    /// Get the position of the `Component` relative to it's parent `Container`.
    ///
    fn get_position(&self) -> Point;

    ///
    /// Set the size of the `Component`. Converts to a `structura_lib::geometry::Size`.
    ///
    fn set_size(&mut self, width: usize, height: usize);

    ///
    /// Gets the size of the `Component` as a `structura_lib::geometry::Size`.
    ///
    fn get_size(&self) -> Size;
}
