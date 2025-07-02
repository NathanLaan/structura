//!
//! Rust MVC-UI
//!

use crate::geometry::Point;
use std::cell::RefCell;
use std::rc::Rc;
use winit::event::MouseScrollDelta;

#[derive(Clone)]
pub struct Callback<Handler> {
    callback: Rc<RefCell<Option<Box<dyn 'static + FnMut(Handler)>>>>,
}

/// Mouse state passed to widgets each frame
#[derive(Clone, Copy, Debug)]
pub struct MouseInput {
    pub position: Point,
    pub pressed: bool,
    pub just_released: bool,
    pub mouse_scroll: Option<MouseScrollDelta>,
}

pub struct KeyboardInput {}
