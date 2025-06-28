//!
//! Structura: GUI Layout components.
//!

use crate::component::Widget;
use crate::event::MouseInput;
use crate::view::BufferContext;

///
/// Layout Trait to position components based on their size and the layout rules.
///
pub trait Layout {
    fn layout(&mut self, container_width: usize, container_height: usize);
}

pub struct Row {
    pub children: Vec<Box<dyn Widget>>,
    pub spacing: usize,
    pub x: usize,
    pub y: usize,
    pub height: usize,
}

impl Row {
    pub fn new(x: usize, y: usize, spacing: usize, height: usize) -> Self {
        Self {
            children: vec![],
            spacing,
            x,
            y,
            height,
        }
    }

    pub fn push<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget));
    }
}

impl Widget for Row {
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

    fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
        self.layout();
    }

    fn set_size(&mut self, _width: usize, height: usize) {
        self.height = height;
        self.layout();
    }
}

impl Row {
    fn layout(&mut self) {
        let mut current_x = self.x;

        for child in self.children.iter_mut() {
            child.set_position(current_x, self.y);
            child.set_size(100, self.height); // fixed width for demo
            current_x += 100 + self.spacing;
        }
    }
}
