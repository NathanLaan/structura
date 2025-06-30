//!
//! Structura
//!
//! Container types
//!

use crate::component::Component;
use crate::event::MouseInput;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;

/// Container node for building widget trees
pub trait Container {
    //fn push<C: Component + 'static>(&mut self, component: C);
    fn push(&mut self, component: Box<dyn Component>);

    ///
    /// Force the `Container` to update the position of controls within the `Container`.
    ///
    fn layout(&mut self);
}

pub trait ContainerComponent: Container + Component {}
impl<T> ContainerComponent for T where T: Container + Component {}

// impl Container {
//     pub fn new() -> Self {
//         Self { children: vec![] }
//     }
//
//     pub fn push<C: Component + 'static>(&mut self, component: C) {
//         self.children.push(Box::new(component));
//     }
// }

// impl Component for Container {
//     fn update(&mut self, input: MouseInput) {
//         for child in self.children.iter_mut() {
//             child.update(input);
//         }
//     }
//
//     fn draw(&self, context: &mut BufferContext) {
//         for child in self.children.iter() {
//             child.draw(context);
//         }
//     }
//
//     fn set_position(&mut self, x: f64, y: f64) {
//         todo!()
//     }
//
//     fn get_position(&self) -> Point {
//         //
//         // TODO: Should be minimum position?
//         //
//         if self.children.is_empty() {
//             Point { x: 0.0, y: 0.0 }
//         } else {
//             Point {
//                 x: self.children[0].get_position().x,
//                 y: self.children[0].get_position().y,
//             }
//         }
//     }
//
//     fn set_size(&mut self, width: usize, height: usize) {
//         todo!()
//     }
//
//     ///
//     /// Returns the size of the `Container`.
//     ///
//     /// The size is defined as the bounding box of all child controls.
//     ///
//     fn get_size(&self) -> Size {
//         if self.children.is_empty() {
//             return Size {
//                 width: 0,
//                 height: 0,
//             };
//         }
//         let mut min_x = self.children[0].get_position().x;
//         let mut min_y = self.children[0].get_position().y;
//         let mut max_x = min_x + self.children[0].get_size().width as f64;
//         let mut max_y = min_y + self.children[0].get_size().height as f64;
//         for component in self.children.iter().skip(1) {
//             min_x = min_x.min(component.get_position().x);
//             min_y = min_y.min(component.get_position().y);
//             max_x = max_x.max(component.get_position().x + component.get_size().width as f64);
//             max_y = max_y.max(component.get_position().y + component.get_size().height as f64);
//         }
//         let max_w = (max_x - min_x).ceil() as u32;
//         let max_h = (max_y - min_y).ceil() as u32;
//         Size {
//             width: max_w,
//             height: max_h,
//         }
//     }
// }

pub struct Row {
    pub children: Vec<Box<dyn Component>>,
    pub spacing: usize,
    pub x: f64,
    pub y: f64,
    pub height: usize,
}

impl Row {
    pub fn new(x: f64, y: f64, spacing: usize, height: usize) -> Self {
        Self {
            children: vec![],
            spacing,
            x,
            y,
            height,
        }
    }

    // pub fn push<W: Component + 'static>(&mut self, widget: W) {
    //     self.children.push(Box::new(widget));
    //     self.layout();
    // }
}

impl Container for Row {
    fn push(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
        self.layout();
    }

    ///
    /// Force the `Layout` to update the position of controls within the `Layout`.
    ///
    /// Layout (position) the controls within the `Row`.
    ///
    fn layout(&mut self) {
        let mut current_x = self.x as f64;
        for child in self.children.iter_mut() {
            child.set_position(current_x, child.get_position().y);
            current_x += child.get_size().width as f64 + self.spacing as f64;
        }
    }
}

impl Component for Row {
    fn update(&mut self, input: MouseInput) {
        for child in self.children.iter_mut() {
            child.update(input);
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
        self.layout();
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
        self.layout();
    }

    ///
    /// Returns the size of the `Row`.
    ///
    /// The size is defined as the bounding box of all child controls.
    ///
    fn get_size(&self) -> Size {
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
        let max_w = (max_x - min_x).ceil() as u32;
        let max_h = (max_y - min_y).ceil() as u32;
        Size {
            width: max_w,
            height: max_h,
        }
    }
}
