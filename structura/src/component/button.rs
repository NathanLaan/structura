//!
//! # Structura Component: Button.
//!
//! A basic clickable button with text.
//!

use crate::component::{Component, ComponentState};
use crate::event::MouseInput;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;
use rusttype::{Scale, point};

///
/// A basic Button component with text.
///
pub struct Button {
    position: Point,
    size: Size,
    text: String,
    component_state: ComponentState,
    on_mouse_over: Option<Box<dyn FnMut()>>,
    on_mouse_click: Option<Box<dyn FnMut()>>,
    mouse_dragging: bool,
}

impl Default for Button {
    ///
    /// Create a default `Button`.
    ///
    fn default() -> Self {
        Self {
            position: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 200,
                height: 60,
            },
            text: "button".to_string(),
            component_state: ComponentState::Active,
            //component_style: ComponentStyle::default(),
            on_mouse_over: None,
            on_mouse_click: None,
            mouse_dragging: false,
        }
    }
}

impl Button {
    ///
    /// Constructor.
    ///
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
            // background_color: 0x0077CC, // blue
            // border_color: 0x000000,
            // border_width: 2,
            text,
            component_state: ComponentState::Active,
            //component_style: ComponentStyle::default(),
            on_mouse_click: None,
            on_mouse_over: None,
            mouse_dragging: false,
        }
    }

    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.position.x
            && px < self.position.x + self.size.width as f64
            && py >= self.position.y
            && py < self.position.y + self.size.height as f64
    }

    ///
    /// Set the `text` displayed on the button.
    ///
    /// TODO: Button contents should be separate from the Button definition, with TextButton and ImageButton/IconButton options.
    ///
    pub fn set_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    // fn update_state(&mut self, cursor_x: f64, cursor_y: f64, mouse_pressed: bool) {
    //     if self.contains(cursor_x, cursor_y) {
    //         self.component_state = if mouse_pressed {
    //             ComponentState::Pressed
    //         } else {
    //             ComponentState::Hovered
    //         };
    //         return;
    //     }
    //     self.component_state = ComponentState::Active;
    // }

    ///
    /// Update `ComponentState` based on mouse position and state.
    ///
    pub fn handle_mouse_event(&mut self, cursor_x: f64, cursor_y: f64, mouse_pressed: bool) {
        if self.contains(cursor_x, cursor_y) {
            if mouse_pressed {
                //println!("Button Pressed: {}", self.text);
                self.component_state = ComponentState::Pressed;
            } else {
                //println!("Button Released: {}", self.text);
                self.component_state = ComponentState::Hovered;
            }
        } else {
            self.component_state = ComponentState::Active;
        }
    }

    ///
    /// Internal function to draw the button. Called by `draw()`.
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
        let bw = context.theme.style_for(&self.component_state).border_width;
        let x0 = self.position.x as usize;
        let y0 = self.position.y as usize;
        let x1 = self.position.x as usize + self.size.width as usize;
        let y1 = self.position.y as usize + self.size.height as usize;
        let fill_x0 = (x0 + bw).min(screen_width);
        let fill_y0 = (y0 + bw).min(screen_height);
        let fill_x1 = x1.saturating_sub(bw).min(screen_width);
        let fill_y1 = y1.saturating_sub(bw).min(screen_height);
        //
        // TODO: Separate ComponentStyles per ComponentState
        //
        let background_color = context
            .theme
            .style_for(&self.component_state)
            .back_color
            .value;
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
        let bw = context.theme.style_for(&self.component_state).border_width;
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
        let border_color = context
            .theme
            .style_for(&self.component_state)
            .border_color
            .value;
        for y in clipped_y0..clipped_y1 {
            for x in clipped_x0..clipped_x1 {
                let is_top = y < y0 + bw;
                let is_bottom = y >= y1.saturating_sub(bw);
                let is_left = x < x0 + bw;
                let is_right = x >= x1.saturating_sub(bw);

                if is_top || is_bottom || is_left || is_right {
                    let idx = y * screen_width + x;
                    if idx < context.buffer.len() {
                        context.buffer[idx] = border_color;
                    }
                }
            }
        }
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

    fn handle_event_on_click(&mut self) {
        match self.component_state {
            ComponentState::Active => {}
            ComponentState::Hovered => {}
            ComponentState::Pressed => {
                if let Some(handler) = self.on_mouse_click.as_mut() {
                    handler();
                }
            }
            ComponentState::Focused => {}
            ComponentState::Disabled => {}
        }
    }

    fn handle_event_on_mouse_over(&mut self) {
        if let Some(handler) = self.on_mouse_over.as_mut() {
            handler();
        }
    }

    ///
    /// Add event handler.
    ///
    pub fn on_click<F: FnMut() + 'static>(mut self, f: F) -> Self {
        self.on_mouse_click = Some(Box::new(f));
        self
    }
}

impl Component for Button {
    fn handle_mouse_event(&mut self, input: MouseInput) {
        //
        // TODO: BUG: Need to be able to handle the case where the button is DISABLED.
        //
        if self.component_state == ComponentState::Disabled {
            return;
        }
        self.component_state = ComponentState::Active;

        if self.contains(input.position.x, input.position.y) {
            self.component_state = ComponentState::Hovered;
            if input.pressed {
                self.component_state = ComponentState::Pressed;
                self.mouse_dragging = true;
            }
            if self.mouse_dragging {
                self.component_state = ComponentState::Pressed;
            }
            if input.just_released {
                //
                // Handle the event before updating ComponentState...
                //
                self.handle_event_on_click();
                self.component_state = ComponentState::Active;
                self.mouse_dragging = false;
            }
            self.handle_event_on_mouse_over();
        }
        if input.just_released {
            self.component_state = ComponentState::Active;
            self.mouse_dragging = false;
        }
    }

    fn handle_mouse_wheel_event(
        &mut self,
        _delta: &winit::event::MouseScrollDelta,
        _phase: &winit::event::TouchPhase,
    ) {
    }

    fn handle_keyboard_event(&mut self, _event: &winit::event::KeyEvent) {
        //
        // Button does nothing on keyboard events.
        //
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
        self.size = Size {
            width: width as u32,
            height: height as u32,
        };
    }

    fn get_size(&self) -> Size {
        self.size.clone()
    }
}
