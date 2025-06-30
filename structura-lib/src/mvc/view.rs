//!
//! Structura. View structures.
//!

use crate::geometry::Size;
use rusttype::Font;
use softbuffer::Buffer;
use std::rc::Rc;
use winit::window::Window;

///
/// A buffer, provided as a context, with associated properties.
///
pub struct BufferContext<'context> {
    pub buffer: Buffer<'context, Rc<Window>, Rc<Window>>,
    pub screen_size: Size,
    pub font: &'context Font<'context>,
    pub font_size: f32,
}
