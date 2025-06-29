//!
//! Rust MVC-UI
//!

pub mod button;
pub mod layout;
pub mod text;
pub mod window;

use crate::component::button::Button;
use crate::event::{Event, MouseInput};
use crate::geometry::Point;
use crate::geometry::Size;
use crate::view::{BufferContext, ViewContext};
use rusttype::Font;

pub fn load_font() -> Font<'static> {
    let font_data = include_bytes!("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    font
}

#[derive(Debug, Clone)]
pub enum ComponentState {
    Active,
    Hovered,
    Pressed,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct ComponentStyle {
    pub text_color: u32,
    pub background_color: u32,
    pub border_color: u32,
}

impl Default for ComponentStyle {
    fn default() -> Self {
        Self {
            background_color: 0x0033CC,
            border_color: 0x000000,
            text_color: 0x000000,
        }
    }
}

/// Widget trait for all UI components
pub trait Widget {
    /// Called each frame to update state (e.g. hover, press)
    fn update(&mut self, input: MouseInput);

    /// Called each frame to render the widget to the pixel buffer
    fn draw(&self, context: &mut BufferContext);

    fn set_position(&mut self, x: f64, y: f64);

    fn get_position(&self) -> Point;

    fn set_size(&mut self, width: usize, height: usize);

    fn get_size(&self) -> Size;
}

///
/// Root GUI Component.
///
pub trait Component<Message> {
    // ///
    // /// Parent control.
    // ///
    // fn parent() -> Self where Self: Sized;
    //
    // ///
    // /// Child controls.
    // ///
    // fn children() -> Vec<Self> where Self: Sized;

    fn update(&mut self, input: MouseInput);

    ///
    /// Draw the component to the screen.
    ///
    fn draw(&self, context: &mut dyn ViewContext);

    ///
    /// Handle an event. Returns a message if triggered.
    ///
    fn handle_event(&mut self, event: &Event) -> Option<Message>;
}

///
/// A Column of elements
///
pub struct Column<Message> {
    ///
    /// The list of components contained within the `Column`.
    ///
    pub children: Vec<Box<dyn Component<Message>>>,
}

/// Container node for building widget trees
pub struct Container {
    pub children: Vec<Box<dyn Widget>>,
}

impl Container {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn push<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget));
    }
}

impl Widget for Container {
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
        todo!()
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

    fn set_size(&mut self, width: usize, height: usize) {
        todo!()
    }

    ///
    /// Returns the size of the `Container`.
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
