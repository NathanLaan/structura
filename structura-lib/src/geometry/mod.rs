//!
//!
//!

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub point: Point,
    pub size: Size,
}
