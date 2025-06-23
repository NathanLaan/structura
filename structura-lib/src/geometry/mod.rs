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
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub point: Point,
    pub size: Size,
}
