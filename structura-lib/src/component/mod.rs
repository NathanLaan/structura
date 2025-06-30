//!
//! Rust MVC-UI
//!

pub mod button;
pub mod layout;
pub mod text;
pub mod window;

use crate::event::MouseInput;
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

/// Widget trait for all UI components
pub trait Widget {
    /// Called each frame to update state (e.g. hover, press)
    fn update(&mut self, input: MouseInput);

    /// Called each frame to render the widget to the pixel buffer
    fn draw(&self, context: &mut BufferContext);

    fn set_position(&mut self, x: f64, y: f64);

    fn get_position(&self) -> Point;

    fn set_size(&mut self, width: usize, height: usize);

    fn get_size(&self) -> Size;
}

// ///
// /// Root GUI Component.
// ///
// pub trait Component { //<Message> {
//     // ///
//     // /// Parent control.
//     // ///
//     // fn parent() -> Self where Self: Sized;
//     //
//     // ///
//     // /// Child controls.
//     // ///
//     // fn children() -> Vec<Self> where Self: Sized;
//
//     fn update(&mut self, input: MouseInput);
//
//     ///
//     /// Draw the component to the screen.
//     ///
//     fn draw(&self, context: &mut BufferContext);
//
//     ///
//     /// Handle an event. Returns a message if triggered.
//     ///
//     fn handle_event(&mut self, event: &Event);
// }

///
/// A Column of elements
///
pub struct Column {}
