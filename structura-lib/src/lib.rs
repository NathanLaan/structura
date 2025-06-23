//!
//! Rust MVC-UI Library
//!

#[path = "mvc/app.rs"]
pub mod app;

#[path = "mvc/view.rs"]
pub mod view;
#[path = "mvc/message.rs"]
pub mod message;

#[path = "draw/primitive.rs"]
pub mod primitive;

pub mod component;

pub mod event;
mod geometry;