//!
//! # Structura Component: Button.
//!
//! A basic button with text.
//!

use crate::component::{ComponentState, ComponentStyle, Component};
use crate::event::{Callback, MouseInput};
use crate::geometry::{Point, Size};
use crate::view::BufferContext;
use rusttype::{Scale, point};
// pub struct ButtonStyle {
//     pub idle: ButtonStateStyle,
//     pub hovered: ButtonStateStyle,
//     pub pressed: ButtonStateStyle,
// }
//
// impl ButtonStyle {
//     pub fn for_state(&self, state: ButtonState) -> &ButtonStateStyle {
//         match state {
//             ButtonState::Idle => &self.idle,
//             ButtonState::Hovered => &self.hovered,
//             ButtonState::Pressed => &self.pressed,
//         }
//     }
// }

///
/// A basic Button component with text.
///
pub struct Button {
    pub position: Point,
    pub size: Size,
    pub background_color: u32,
    pub border_color: u32,
    pub border_width: usize,
    pub text: String,
    pub component_state: ComponentState,
    pub component_style: ComponentStyle,
    pub on_clicked: Option<Callback<()>>,
    /// Optional event handler
    on_click: Option<Box<dyn FnMut()>>,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            position: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 200,
                height: 60,
            },
            background_color: 0x0077CC, // blue
            border_color: 0x000000,
            border_width: 2,
            text: "button".to_string(),
            component_state: ComponentState::Active,
            component_style: ComponentStyle {
                background_color: 0x0033CC,
                border_color: 0x000000,
                text_color: 0x000000,
            },
            on_click: None,
            on_clicked: None,
        }
    }
}

impl Button {
    pub fn new(x: usize, y: usize, width: usize, height: usize, text: String) -> Self {
        Self {
            position: Point {
                x: x as f64,
                y: y as f64,
            },
            size: Size {
                width: width as u32,
                height: height as u32,
            },
            background_color: 0x0077CC, // blue
            border_color: 0x000000,
            border_width: 2,
            text,
            component_state: ComponentState::Active,
            component_style: ComponentStyle::default(),
            on_click: None,
            on_clicked: None,
        }
    }

    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.position.x
            && px < self.position.x + self.size.width as f64
            && py >= self.position.y
            && py < self.position.y + self.size.height as f64
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn update_state(&mut self, cursor_x: f64, cursor_y: f64, mouse_pressed: bool) {
        if self.contains(cursor_x, cursor_y) {
            self.component_state = if mouse_pressed {
                ComponentState::Pressed
            } else {
                ComponentState::Hovered
            };
            return;
        }
        self.component_state = ComponentState::Active;
    }

    ///
    /// Update `ComponentState` based on mouse position and state.
    ///
    pub fn handle_mouse_event(&mut self, cursor_x: f64, cursor_y: f64, mouse_pressed: bool) {
        if self.contains(cursor_x, cursor_y) {
            if mouse_pressed {
                println!("Button Pressed: {}", self.text);
                self.component_state = ComponentState::Pressed;
            } else {
                println!("Button Released: {}", self.text);
                self.component_state = ComponentState::Hovered;
            }
        } else {
            self.component_state = ComponentState::Active;
        }
    }

    ///
    /// TODO: Font and font_size should come from theme/settings.
    ///
    ///
    fn draw_button(&self, context: &mut BufferContext) {
        self.fill_background(context);
        self.draw_border(context);

        // Text rendering parameters
        let font_scale = Scale::uniform(context.font_size);
        let v_metrics = context.font.v_metrics(font_scale);

        let screen_width = context.screen_size.width as usize;

        let start_x = self.position.x + 10.0;
        let start_y =
            self.position.y + (self.size.height as f64 / 2.0) + (v_metrics.ascent / 2.0) as f64;

        //
        // TODO: Move to text rendering component
        //
        let glyphs: Vec<_> = context
            .font
            .layout(
                &self.text[..],
                font_scale,
                point(start_x as f32, start_y as f32),
            )
            .collect();

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let x = gx as i32 + bb.min.x;
                    let y = gy as i32 + bb.min.y;
                    if x >= 0
                        && x < context.screen_size.width as i32
                        && y >= 0
                        && (y as usize) < context.buffer.len() / screen_width
                    {
                        let idx = y as usize * screen_width + x as usize;
                        context.buffer[idx] = Self::basic_aa(context.buffer[idx], 0xFFFFFF, v);
                    }
                });
            }
        }
    }

    ///
    /// Fill in the background of the Button.
    ///
    fn fill_background(&self, context: &mut BufferContext) {
        let screen_width = context.screen_size.width as usize;
        let screen_height = context.screen_size.height as usize;
        let bw = self.border_width;
        let x0 = self.position.x as usize;
        let y0 = self.position.y as usize;
        let x1 = self.position.x as usize + self.size.width as usize;
        let y1 = self.position.y as usize + self.size.height as usize;
        let fill_x0 = (x0 + bw).min(screen_width);
        let fill_y0 = (y0 + bw).min(screen_height);
        let fill_x1 = x1.saturating_sub(bw).min(screen_width);
        let fill_y1 = y1.saturating_sub(bw).min(screen_height);
        let background_color = match self.component_state {
            ComponentState::Active => self.component_style.background_color,
            ComponentState::Hovered => 0x0077CC,
            ComponentState::Pressed => 0x0099CC,
            ComponentState::Disabled => 0xCCCCCC,
        };
        for y in fill_y0..fill_y1 {
            for x in fill_x0..fill_x1 {
                let idx = y * screen_width + x;
                if idx < context.buffer.len() {
                    context.buffer[idx] = background_color;
                }
            }
        }
    }

    ///
    /// Draw the Button border.
    ///
    fn draw_border(&self, context: &mut BufferContext) {
        let bw = self.border_width;
        let x0 = self.position.x as usize;
        let y0 = self.position.y as usize;
        let x1 = self.position.x as usize + self.size.width as usize;
        let y1 = self.position.y as usize + self.size.height as usize;
        let screen_width = context.screen_size.width as usize;
        let screen_height = context.screen_size.height as usize;
        let clipped_x0 = x0.min(screen_width);
        let clipped_y0 = y0.min(screen_height);
        let clipped_x1 = x1.min(screen_width);
        let clipped_y1 = y1.min(screen_height);
        for y in clipped_y0..clipped_y1 {
            for x in clipped_x0..clipped_x1 {
                let is_top = y < y0 + bw;
                let is_bottom = y >= y1.saturating_sub(bw);
                let is_left = x < x0 + bw;
                let is_right = x >= x1.saturating_sub(bw);

                if is_top || is_bottom || is_left || is_right {
                    let idx = y * screen_width + x;
                    if idx < context.buffer.len() {
                        context.buffer[idx] = self.border_color;
                    }
                }
            }
        }
    }

    fn blend_pixel(bg: u32, brightness: u8) -> u32 {
        let r = brightness as u32;
        let g = brightness as u32;
        let b = brightness as u32;
        (r << 16) | (g << 8) | b
    }

    fn basic_aa(bg: u32, fg: u32, alpha: f32) -> u32 {
        let inv = 1.0 - alpha;

        let br = ((bg >> 16) & 0xFF) as f32;
        let bg_ = ((bg >> 8) & 0xFF) as f32;
        let bb = (bg & 0xFF) as f32;

        let fr = ((fg >> 16) & 0xFF) as f32;
        let fg_ = ((fg >> 8) & 0xFF) as f32;
        let fb = (fg & 0xFF) as f32;

        let r = (br * inv + fr * alpha).round() as u32;
        let g = (bg_ * inv + fg_ * alpha).round() as u32;
        let b = (bb * inv + fb * alpha).round() as u32;

        (r << 16) | (g << 8) | b
    }

    fn handle_event(&mut self) {
        match self.component_state {
            ComponentState::Active => {}
            ComponentState::Hovered => {}
            ComponentState::Pressed => {
                self.event_button_pressed();
            }
            ComponentState::Disabled => {}
        }
    }

    fn event_button_pressed(&mut self) {
        println!("Button Pressed {}", self.text);
        if let Some(handler) = self.on_click.as_mut() {
            handler(); // 🔥 callback fired
        }
    }
    fn event_button_released(&self) {
        println!("Button Released {}", self.text);
    }

    ///
    /// Add event handler.
    ///
    pub fn on_click<F: FnMut() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl Component for Button {
    fn update(&mut self, input: MouseInput) {
        if self.contains(input.position.x, input.position.y) {
            self.component_state = if input.pressed {
                ComponentState::Pressed
            } else {
                ComponentState::Hovered
            };
            if input.pressed {
                self.handle_event();
            }
            return;
        }
        self.component_state = ComponentState::Active;
    }

    fn draw(&self, context: &mut BufferContext) {
        self.draw_button(context);
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.position = Point { x, y };
    }

    fn get_position(&self) -> Point {
        self.position
    }

    fn set_size(&mut self, width: usize, height: usize) {
        todo!()
    }

    fn get_size(&self) -> Size {
        self.size.clone()
    }
}
