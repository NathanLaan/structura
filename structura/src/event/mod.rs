//!
//! Structura
//!

use crate::geometry::Point;
use winit::event::MouseScrollDelta;

// #[derive(Clone)]
// pub struct Callback<Handler> {
//     callback: Rc<RefCell<Option<Box<dyn 'static + FnMut(Handler)>>>>,
// }

///
/// Mouse state for mouse events.
///
#[derive(Clone, Copy, Debug)]
pub struct MouseInput {
    pub scale_factor: f64,
    pub position: Point,
    pub pressed: bool,
    pub just_released: bool,
    pub mouse_scroll: Option<MouseScrollDelta>,
}
