//!
//! Structure: Row ContainerComponent.
//!

use crate::component::Component;
use crate::container::Container;
use crate::event::MouseInput;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;

///
/// Horizontal Row of components.
///
pub struct Row {
    pub children: Vec<Box<dyn Component>>,
    pub spacing: usize,
    pub x: f64,
    pub y: f64,
    pub height: usize,
}

impl Row {
    ///
    /// Constructor.
    ///
    pub fn new(x: f64, y: f64, spacing: usize, height: usize) -> Self {
        Self {
            children: vec![],
            spacing,
            x,
            y,
            height,
        }
    }
}

impl Container for Row {
    ///
    /// Add the specified `Component` as a child to the `Container`.
    ///
    fn push(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
        self.update_layout();
    }
    fn fills_parent_container(&self) -> bool {
        false
    }

    ///
    /// Called when the parent `Container` resizes.
    ///
    /// Passes in the parent container `width` and `height`.
    ///
    /// The `Row` does not change its `height` or `width` based on parent changes.
    ///
    fn resize(&mut self, _width: usize, _height: usize) {
        // for child in self.children.iter_mut() {
        //     child.resize(width, height);
        // }
    }

    ///
    /// Force the `Layout` to update the position of controls within the `Layout`.
    ///
    /// Layout (position) the controls within the `Row`.
    ///
    fn update_layout(&mut self) {
        let mut current_x = self.x as f64;
        let current_y = self.y as f64;
        for child in self.children.iter_mut() {
            child.set_position(current_x, current_y);
            current_x += child.get_size().width as f64 + self.spacing as f64;
        }
    }
}

impl Component for Row {
    fn handle_mouse_event(&mut self, input: MouseInput) {
        for child in self.children.iter_mut() {
            child.handle_mouse_event(input);
        }
    }

    fn handle_mouse_wheel_event(
        &mut self,
        delta: &winit::event::MouseScrollDelta,
        phase: &winit::event::TouchPhase,
    ) {
        for child in self.children.iter_mut() {
            child.handle_mouse_wheel_event(&delta, &phase);
        }
    }

    fn handle_keyboard_event(&mut self, event: &winit::event::KeyEvent) {
        for child in self.children.iter_mut() {
            child.handle_keyboard_event(event);
        }
    }

    fn draw(&self, context: &mut BufferContext) {
        for child in self.children.iter() {
            child.draw(context);
        }
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.update_layout();
    }

    fn get_position(&self) -> Point {
        //
        // TODO: Should be minimum position?
        //
        if self.children.is_empty() {
            Point { x: 0.0, y: 0.0 }
        } else {
            Point {
                x: self.children[0].get_position().x,
                y: self.children[0].get_position().y,
            }
        }
    }

    fn set_size(&mut self, _width: usize, height: usize) {
        self.height = height;
        self.update_layout();
    }

    ///
    /// Returns the size of the `Row`.
    ///
    /// The size is defined as the bounding box of all child controls.
    ///
    fn get_size(&self) -> Size {
        //
        // TODO: Add margin
        //
        if self.children.is_empty() {
            return Size {
                width: 0,
                height: 0,
            };
        }
        let mut min_x = self.children[0].get_position().x;
        let mut min_y = self.children[0].get_position().y;
        let mut max_x = min_x + self.children[0].get_size().width as f64;
        let mut max_y = min_y + self.children[0].get_size().height as f64;
        for component in self.children.iter().skip(1) {
            min_x = min_x.min(component.get_position().x);
            min_y = min_y.min(component.get_position().y);
            max_x = max_x.max(component.get_position().x + component.get_size().width as f64);
            max_y = max_y.max(component.get_position().y + component.get_size().height as f64);
        }
        let max_w = (max_x - min_x).ceil() as u32 + (self.spacing * 2) as u32;
        let max_h = (max_y - min_y).ceil() as u32 + (self.spacing * 2) as u32;
        Size {
            width: max_w,
            height: max_h,
        }
    }
}
