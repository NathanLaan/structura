//!
//! Structura: Geometric Types.
//!

///
/// Represents an `{x,y}` point in 2D space.
///
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

///
/// Represents an object in 2D space with size `{width,height}`.
///
#[derive(Debug, Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

///
/// A rectangular shape defined by a `Point` and `Size` in 2D space.
///
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub point: Point,
    pub size: Size,
}
