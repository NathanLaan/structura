//!
//! Structura. View types.
//!

use crate::component::style::ComponentTheme;
use crate::geometry::Size;
use rusttype::Font;
use softbuffer::Buffer;
use std::rc::Rc;
use winit::window::Window;

///
/// A drawing buffer, provided as a context, with associated fields.
///
pub struct BufferContext<'context> {
    pub buffer: Buffer<'context, Rc<Window>, Rc<Window>>,
    pub screen_size: Size,
    pub font: &'context Font<'context>,
    pub font_size: f32,
    pub theme: &'context Box<dyn ComponentTheme>,
}
