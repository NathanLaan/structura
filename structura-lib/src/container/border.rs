//!
//!
//!

use crate::component::Component;
use crate::container::{Container, ContainerComponent};
use crate::event::MouseInput;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;
use winit::event::{KeyEvent, MouseScrollDelta, TouchPhase};

pub struct BorderLayout {
    pub east: Option<Box<dyn ContainerComponent>>,
    pub west: Option<Box<dyn ContainerComponent>>,
    pub north: Option<Box<dyn ContainerComponent>>,
    pub south: Option<Box<dyn ContainerComponent>>,
    pub center: Option<Box<dyn ContainerComponent>>,
}

impl Container for BorderLayout {
    fn push(&mut self, component: Box<dyn Component>) {}
    fn update_layout(&mut self) {
        todo!()
    }
}

impl Component for BorderLayout {
    fn handle_mouse_event(&mut self, input: MouseInput) {
        if let Some(comp) = self.east.as_mut() {
            comp.handle_mouse_event(input);
        }
        if let Some(comp) = self.west.as_mut() {
            comp.handle_mouse_event(input);
        };
        if let Some(comp) = self.north.as_mut() {
            comp.handle_mouse_event(input);
        }
        if let Some(comp) = self.south.as_mut() {
            comp.handle_mouse_event(input);
        }
        if let Some(comp) = self.center.as_mut() {
            comp.handle_mouse_event(input);
        }
    }

    fn handle_mouse_wheel_event(
        &mut self,
        delta: &winit::event::MouseScrollDelta,
        phase: &winit::event::TouchPhase,
    ) {
        if let Some(comp) = self.east.as_mut() {
            comp.handle_mouse_wheel_event(delta, phase);
        }
        if let Some(comp) = self.west.as_mut() {
            comp.handle_mouse_wheel_event(delta, phase);
        };
        if let Some(comp) = self.north.as_mut() {
            comp.handle_mouse_wheel_event(delta, phase);
        }
        if let Some(comp) = self.south.as_mut() {
            comp.handle_mouse_wheel_event(delta, phase);
        }
        if let Some(comp) = self.center.as_mut() {
            comp.handle_mouse_wheel_event(delta, phase);
        }
    }

    fn handle_keyboard_event(&mut self, event: &winit::event::KeyEvent) {
        if let Some(comp) = self.east.as_mut() {
            comp.handle_keyboard_event(event);
        }
        if let Some(comp) = self.west.as_mut() {
            comp.handle_keyboard_event(event);
        };
        if let Some(comp) = self.north.as_mut() {
            comp.handle_keyboard_event(event);
        }
        if let Some(comp) = self.south.as_mut() {
            comp.handle_keyboard_event(event);
        }
        if let Some(comp) = self.center.as_mut() {
            comp.handle_keyboard_event(event);
        }
    }

    fn draw(&self, context: &mut BufferContext) {
        if let Some(comp) = self.east.as_deref() {
            comp.draw(context);
        }
        if let Some(comp) = self.west.as_deref() {
            comp.draw(context);
        };
        if let Some(comp) = self.north.as_deref() {
            comp.draw(context);
        }
        if let Some(comp) = self.south.as_deref() {
            comp.draw(context);
        }
        if let Some(comp) = self.center.as_deref() {
            comp.draw(context);
        }
    }

    fn set_position(&mut self, x: f64, y: f64) {
        // self.position.x = x;
        // self.position.y = y;
        self.update_layout();
    }

    fn get_position(&self) -> Point {
        todo!()
    }

    fn set_size(&mut self, width: usize, height: usize) {
        todo!()
    }

    fn get_size(&self) -> Size {
        todo!()
    }
}
