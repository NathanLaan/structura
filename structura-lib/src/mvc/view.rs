//!
//! Rust MVC-UI
//!
use crate::control::Component;
use crate::message::Message;


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