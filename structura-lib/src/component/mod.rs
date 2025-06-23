//!
//! Rust MVC-UI
//!

pub mod button;
pub mod text;
pub mod window;

use crate::event::Event;
use crate::view::ViewContext;

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
