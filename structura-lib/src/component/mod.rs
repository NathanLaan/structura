//!
//! Rust MVC-UI
//!

pub mod button;
pub mod text;
pub mod window;

use crate::event::Event;
use crate::view::ViewContext;
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

///
/// Root GUI Component.
///
pub trait Component<Message> {
    // ///
    // /// Parent control.
    // ///
    // fn parent() -> Self where Self: Sized;
    //
    // ///
    // /// Child controls.
    // ///
    // fn children() -> Vec<Self> where Self: Sized;

    ///
    /// Draw the component to the screen.
    ///
    fn draw(&self, context: &mut dyn ViewContext);

    ///
    /// Handle an event. Returns a message if triggered.
    ///
    fn handle_event(&mut self, event: &Event) -> Option<Message>;
}

///
/// A Column of elements
///
pub struct Column<Message> {
    ///
    /// The list of components contained within the `Column`.
    ///
    pub children: Vec<Box<dyn Component<Message>>>,
}
