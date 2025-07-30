//!
//! **Structura**: Panel
//!

use crate::component::Component;
use crate::container::Container;
use crate::event::MouseInput;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;

///
/// A Container that holds a single component, and resizes to fit the bounds of its parent.
///
pub struct Panel {
    position: Point,
    size: Size,
    component: Option<Box<dyn Component>>,
}

impl Panel {
    ///
    /// Constructor for `Panel`.
    ///
    pub fn new() -> Self {
        Self {
            position: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 0,
                height: 0,
            },
            component: None,
        }
    }
}

impl Container for Panel {
    fn push(&mut self, component: Box<dyn Component>) {
        self.component = Some(component);
    }

    fn fills_parent_container(&self) -> bool {
        true
    }

    ///
    /// Called when the parent `Container` resizes.
    ///
    /// Passes in the parent container `width` and `height`.
    ///
    fn resize(&mut self, width: usize, height: usize) {
        self.size.width = width as u32;
        self.size.height = height as u32;
        self.update_layout();
    }
    fn update_layout(&mut self) {
        if let Some(comp) = self.component.as_mut() {
            comp.set_position(self.position.x, self.position.y);
            comp.set_size(self.size.width as usize, self.size.height as usize);
        }
    }
}

impl Component for Panel {
    fn handle_mouse_event(&mut self, input: MouseInput) {
        if let Some(comp) = self.component.as_mut() {
            comp.handle_mouse_event(input);
        }
    }

    fn handle_mouse_wheel_event(
        &mut self,
        delta: &winit::event::MouseScrollDelta,
        phase: &winit::event::TouchPhase,
    ) {
        if let Some(comp) = self.component.as_mut() {
            comp.handle_mouse_wheel_event(delta, phase);
        }
    }

    fn handle_keyboard_event(&mut self, event: &winit::event::KeyEvent) {
        if let Some(comp) = self.component.as_mut() {
            comp.handle_keyboard_event(event);
        }
    }

    fn draw(&self, context: &mut BufferContext) {
        if let Some(comp) = self.component.as_deref() {
            comp.draw(context);
        }
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.position.x = x;
        self.position.y = y;
        self.update_layout();
    }

    fn get_position(&self) -> Point {
        self.position
    }

    fn set_size(&mut self, width: usize, height: usize) {
        self.size.width = width as u32;
        self.size.height = height as u32;
    }

    fn get_size(&self) -> Size {
        self.size.clone()
    }
}
