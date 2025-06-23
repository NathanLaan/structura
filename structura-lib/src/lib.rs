//!
//! Rust MVC-UI Library
//!

#[path = "mvc/app.rs"]
pub mod app;

#[path = "mvc/message.rs"]
pub mod message;
#[path = "mvc/view.rs"]
pub mod view;

#[path = "draw/device.rs"]
pub mod device;
#[path = "draw/primitive.rs"]
pub mod primitive;
#[path = "draw/render.rs"]
pub mod render;

pub mod component;

pub mod event;
pub mod geometry;
