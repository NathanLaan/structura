//!
//! Rust MVC-UI
//!

use crate::component::Component;
use crate::message::Message;
use rusttype::Font;
use softbuffer::Buffer;
use std::rc::Rc;
use winit::window::Window;

pub struct BufferContext<'context> {
    //pub buffer: &'context mut [u32],
    pub buffer: Buffer<'context, Rc<Window>, Rc<Window>>,
    pub screen_width: usize,
    pub screen_height: usize,
    pub font: &'context Font<'context>,
    pub font_size: f32,
}

pub trait ViewContext {
    fn draw_text(&mut self, text: &str, x: i32, y: i32);
    fn draw_button(&mut self, label: &str, x: i32, y: i32, hovered: bool);
}

///
/// The View
///
pub trait View {
    fn view(&self) -> dyn Component<dyn Message>;
}
