//!
//! **Structura** GUI Components.
//!

pub mod button;
pub mod label;
pub mod style;
pub mod text;
pub mod textarea;
mod tooltip;

use crate::event::MouseInput;
use crate::geometry::Point;
use crate::geometry::Size;
use crate::view::BufferContext;
use rusttype::Font;
use std::cell::RefCell;
use std::rc::Rc;
use winit::event::MouseScrollDelta;
use winit::event::{KeyEvent, TouchPhase};

///
/// Utility function to load a `rusttype::Font`.
///
/// TODO: Fonts needs to be a fundamental property of `Component`s that display text.
///
pub fn load_font() -> Font<'static> {
    let font_data = include_bytes!("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    font
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ComponentState {
    Active,
    Hovered,
    Pressed,
    Focused,
    Disabled,
}

///
/// Base trait for all UI components
///
pub trait Component {
    ///
    /// Called whenever the user makes an input (e.g. mouse moved, mouse pressed).
    ///
    fn handle_mouse_event(&mut self, input: MouseInput);

    ///
    /// Called whenever the user make a MouseWheel input.
    ///
    fn handle_mouse_wheel_event(
        &mut self,
        event: &winit::event::MouseScrollDelta,
        phase: &winit::event::TouchPhase,
    );

    fn handle_keyboard_event(&mut self, event: &winit::event::KeyEvent);

    ///
    /// Called whenever the application updates it's state, or a redraw is requested in response to an event.
    ///
    fn draw(&self, context: &mut BufferContext);

    ///
    /// Set the position of the `Component` relative to it's parent `Container`.
    ///
    fn set_position(&mut self, x: f64, y: f64);

    ///
    /// Get the position of the `Component` relative to it's parent `Container`.
    ///
    fn get_position(&self) -> Point;

    ///
    /// Set the size of the `Component`. Converts to a `structura_lib::geometry::Size`.
    ///
    fn set_size(&mut self, width: usize, height: usize);

    ///
    /// Gets the size of the `Component` as a `structura_lib::geometry::Size`.
    ///
    fn get_size(&self) -> Size;
}

///
/// `Rc<RefCell<T>>` wrapper for structs that implement the `Component` trait.
///
pub struct ComponentHandle<T: Component> {
    inner: Rc<RefCell<T>>,
}

impl<T: Component> ComponentHandle<T> {
    pub fn new(inner: Rc<RefCell<T>>) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> Rc<RefCell<T>> {
        self.inner.clone()
    }
}

impl<T: Component> Component for ComponentHandle<T> {
    fn handle_mouse_event(&mut self, input: MouseInput) {
        self.inner.borrow_mut().handle_mouse_event(input);
    }

    fn handle_mouse_wheel_event(&mut self, event: &MouseScrollDelta, phase: &TouchPhase) {
        self.inner
            .borrow_mut()
            .handle_mouse_wheel_event(event, phase);
    }

    fn handle_keyboard_event(&mut self, event: &KeyEvent) {
        self.inner.borrow_mut().handle_keyboard_event(event);
    }

    fn draw(&self, context: &mut BufferContext) {
        self.inner.borrow().draw(context);
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.inner.borrow_mut().set_position(x, y);
    }

    fn get_position(&self) -> Point {
        self.inner.borrow().get_position()
    }

    fn set_size(&mut self, width: usize, height: usize) {
        self.inner.borrow_mut().set_size(width, height);
    }

    fn get_size(&self) -> Size {
        self.inner.borrow().get_size()
    }
}
