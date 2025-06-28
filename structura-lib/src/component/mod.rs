//!
//! Rust MVC-UI
//!

pub mod button;
pub mod text;
pub mod window;

use crate::component::button::Button;
use crate::event::{Event, MouseInput};
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

/// Widget trait for all UI components
pub trait Widget {
    /// Called each frame to update state (e.g. hover, press)
    fn update(&mut self, input: MouseInput);

    /// Called each frame to render the widget to the pixel buffer
    fn draw(&self, context: &mut BufferContext);

    /// Optional click signal (for buttons, etc.)
    fn was_clicked(&self) -> bool {
        false
    }
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

impl Widget for Button {
    fn update(&mut self, input: MouseInput) {
        //self.was_clicked = false;
        if let Some((mx, my)) = input.position {
            if self.contains(mx, my) {
                if input.just_released {
                    //self.was_clicked = true;
                }
                self.component_state = if input.pressed {
                    ComponentState::Pressed
                } else {
                    ComponentState::Hovered
                };
                return;
            }
        }
        self.component_state = ComponentState::Active;
    }

    fn draw(&self, context: &mut BufferContext) {
        let bw = self.border_width;
        let x0 = self.x.min(context.screen_width);
        let y0 = self.y.min(context.screen_height);
        let x1 = (self.x + self.width).min(context.screen_width);
        let y1 = (self.y + self.height).min(context.screen_height);

        let fill = self.background_color;

        for y in y0..y1 {
            for x in x0..x1 {
                let idx = y * context.screen_width + x;
                if x < x0 + bw || x >= x1 - bw || y < y0 + bw || y >= y1 - bw {
                    context.buffer[idx] = self.border_color;
                } else {
                    context.buffer[idx] = self.background_color;
                }
            }
        }
        // Text drawing omitted for brevity
    }

    fn was_clicked(&self) -> bool {
        //self.was_clicked
        false
    }
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
}
