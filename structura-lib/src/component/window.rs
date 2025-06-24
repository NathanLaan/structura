//!
//!
//!
use crate::geometry::{Point, Rectangle, Size};

pub type WindowHandle = i64;

pub struct WindowSettings {
    pub title: String,
}

pub struct Window {
    pub size: Size,
    pub location: Point,
}
