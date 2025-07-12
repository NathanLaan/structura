//!
//! # **Structura**: BorderLayout
//!
//! Implements `Container` and `Component`.
///
/// |------------------------|
/// | North                  |
/// |------------------------|
/// |      |          |      |
/// | West |  Center  | East |
/// |      |          |      |
/// |------------------------|
/// | South                  |
/// |------------------------|
///
/// ## Resize rules
///
/// - Center: Resizes `width` and `height` when `BorderLayout` resizes.
/// - North: Resizes `width` when `BorderLayout` resizes.
/// - South: Resizes `width` when `BorderLayout` resizes.
/// - West: Resizes `height` when `BorderLayout` resizes.
/// - East: Resizes `height` when `BorderLayout` resizes.
///
use crate::component::Component;
use crate::container::{Container, ContainerComponent};
use crate::event::MouseInput;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;

///
/// BorderLayout: Implements `Container` and `Component`.
///
pub struct BorderLayout {
    position: Point,
    size: Size,
    east: Option<Box<dyn ContainerComponent>>,
    west: Option<Box<dyn ContainerComponent>>,
    north: Option<Box<dyn ContainerComponent>>,
    south: Option<Box<dyn ContainerComponent>>,
    center: Option<Box<dyn ContainerComponent>>,
}

impl BorderLayout {
    pub fn new() -> Self {
        Self {
            position: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 0,
                height: 0,
            },
            east: None,
            west: None,
            north: None,
            south: None,
            center: None,
        }
    }
    pub fn set_east(&mut self, east: Box<dyn ContainerComponent>) {
        self.east = Some(east);
    }
    pub fn set_west(&mut self, east: Box<dyn ContainerComponent>) {
        self.west = Some(east);
    }
    pub fn set_north(&mut self, east: Box<dyn ContainerComponent>) {
        self.north = Some(east);
    }
    pub fn set_south(&mut self, east: Box<dyn ContainerComponent>) {
        self.south = Some(east);
    }
    pub fn set_center(&mut self, east: Box<dyn ContainerComponent>) {
        self.center = Some(east);
    }
}

impl Container for BorderLayout {
    fn push(&mut self, _component: Box<dyn Component>) {
        //
        // TODO: Return an error?
        //
        // TODO: Cycle through and add: N -> W -> C -> E -> S.
        //
    }

    ///
    /// The `BorderLayout` always fills it's parent `Container`.
    ///
    fn fills_parent_container(&self) -> bool {
        true
    }

    ///
    /// Called when the parent `Container` resizes.
    ///
    /// Passes in the parent container `width` and `height`.
    ///
    fn resize(&mut self, width: usize, height: usize) {
        println!("BorderLayout::resize");
        self.size.width = width as u32;
        self.size.height = height as u32;
        self.update_layout();
    }
    fn update_layout(&mut self) {
        let ww = if let Some(comp) = self.west.as_mut() {
            comp.get_size().width
        } else {
            0
        };
        let ew = if let Some(comp) = self.east.as_mut() {
            comp.get_size().width
        } else {
            0
        };
        let nh = if let Some(comp) = self.north.as_mut() {
            comp.get_size().height
        } else {
            0
        };
        let sh = if let Some(comp) = self.south.as_mut() {
            comp.get_size().height
        } else {
            0
        };

        let cw = self.size.width - ww - ew;
        let ch = self.size.height - nh - sh;

        if let Some(comp) = self.east.as_mut() {
            comp.resize(comp.get_size().width as usize, self.size.height as usize);
        }
        if let Some(comp) = self.west.as_mut() {
            comp.resize(comp.get_size().width as usize, self.size.height as usize);
        };
        if let Some(comp) = self.north.as_mut() {
            comp.set_position(self.position.x, self.position.y);
            comp.resize(self.size.width as usize, comp.get_size().height as usize);
        }
        if let Some(comp) = self.south.as_mut() {
            comp.resize(self.size.width as usize, comp.get_size().height as usize);
        }
        if let Some(comp) = self.center.as_mut() {
            comp.set_position(ww as f64, nh as f64);
            comp.resize(cw as usize, ch as usize);
        }
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
