//!
//! Rust MVC-UI
//!

use crate::geometry::Point;
use std::cell::RefCell;
use std::rc::{Rc};

/// Mouse state passed to widgets each frame
#[derive(Clone, Copy, Debug)]
pub struct MouseInput {
    pub position: Point,
    pub pressed: bool,
    pub just_released: bool,
}

#[derive(Debug)]
pub enum Event {
    MouseClick { x: i32, y: i32 },
    KeyPress(char),
}

#[derive(Clone)]
pub struct Callback<Handler> {
    callback: Rc<RefCell<Option<Box<dyn 'static + FnMut(Handler)>>>>,
}
