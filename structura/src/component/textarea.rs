//!
//! # Structura Component: TextArea.
//!
//! A basic editable, multiline text `Component`.
//!

use crate::component::style::ColorFactor;
use crate::component::{Component, ComponentState};
use crate::event::MouseInput;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;
use rusttype::{PositionedGlyph, Scale, point};
use winit::keyboard::{Key, NamedKey};

///
/// TextArea control for displaying editable multi-line, scrollable text.
///
/// - TODO: Horizontal scrolling.
/// - TODO: Named field to toggle multiline.
/// - TODO: Named field to disable scrolling.
/// - TODO: Named field to disable editing.
///
pub struct TextArea {
    text: String,
    pub cursor_index: usize,
    pub position: Point,
    pub size: Size,
    pub focused: bool,
    component_state: ComponentState,
    visible_scrolling_offset: f32,
    pub dragging_scrollbar: bool,
    pub last_mouse_y: Option<f64>,
    scroll_amount_y: f32,
    on_text_change: Option<Box<dyn FnMut()>>,
    scrollbar_width: usize,
}

impl Clone for TextArea {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            cursor_index: self.cursor_index.clone(),
            position: self.position.clone(),
            size: self.size.clone(),
            focused: self.focused.clone(),
            component_state: self.component_state.clone(),
            visible_scrolling_offset: self.visible_scrolling_offset.clone(),
            dragging_scrollbar: self.dragging_scrollbar.clone(),
            last_mouse_y: self.last_mouse_y.clone(),
            scroll_amount_y: self.scroll_amount_y.clone(),
            on_text_change: None, // Cannot clone!
            scrollbar_width: self.scrollbar_width.clone(),
        }
    }
}

impl TextArea {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor_index: 0,
            position: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 200,
                height: 50,
            },
            focused: false,
            component_state: ComponentState::Active,
            visible_scrolling_offset: 0.0,
            dragging_scrollbar: false,
            last_mouse_y: None,
            scroll_amount_y: 10.0,
            on_text_change: None,
            scrollbar_width: 20,
        }
    }

    ///
    /// Add event handler.
    ///
    pub fn on_text_change<F: FnMut() + 'static>(mut self, f: F) -> Self {
        self.on_text_change = Some(Box::new(f));
        self
    }

    ///
    /// Internal: Call `self.on_text_change` event handler.
    ///
    fn handle_event(&mut self) {
        match self.component_state {
            ComponentState::Active | ComponentState::Focused => {
                if let Some(handler) = self.on_text_change.as_mut() {
                    handler();
                }
            }
            ComponentState::Disabled => {}
            ComponentState::Hovered => {}
            ComponentState::Pressed => {}
        }
    }

    ///
    /// Insert the specified `String`, update the cursor position, and trigger
    /// the TextArea `on_text_change()` event.
    ///
    pub fn insert_str(&mut self, str: &str) {
        self.text.insert_str(self.cursor_index, str);
        self.cursor_index += str.len();
        self.handle_event();
    }

    ///
    /// Checks if the specified `px` and `py` coordinates are inside the
    /// bounding box of the `TextArea`.
    ///
    fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.position.x
            && x < self.position.x + self.size.width as f64
            && y >= self.position.y
            && y < self.position.y + self.size.height as f64
    }

    ///
    /// Checks if the specified `x` and `y` coordinates are inside the
    /// bounding box of the scrollbar for the `TextArea`.
    ///
    fn scrollbar_contains(&self, x: f64, y: f64) -> bool {
        let scroll_x = self.position.x + (self.size.width - self.scrollbar_width as u32) as f64;
        x >= scroll_x
            && x <= scroll_x + self.scrollbar_width as f64
            && y >= self.position.y
            && y <= self.position.y + self.size.height as f64
    }

    fn draw_background(&self, context: &mut BufferContext) {
        let px = self.position.x as usize;
        let py = self.position.y as usize;
        let w = self.size.width;
        let h = self.size.height;
        let screen_w = context.screen_size.width as usize;

        // Background
        for y in 0..h {
            for x in 0..w {
                let offset = (py + y as usize) * screen_w + (px + x as usize);
                if offset < context.buffer.len() {
                    //
                    // TODO: Move to dedicated style variables
                    //
                    context.buffer[offset] = if self.focused {
                        0xFFEEEEFF // background when focused
                    } else {
                        0xFFFFFFFF // background when not focused
                    };
                }
            }
        }
    }

    ///
    /// Draws the `text` and returns the number of lines of text.
    ///
    fn draw_text(&self, context: &mut BufferContext) -> usize {
        let font_scale = Scale::uniform(context.font_size);
        let v_metrics = context.font.v_metrics(font_scale);
        let line_height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap).ceil();
        let space_width = context
            .font
            .glyph(' ')
            .scaled(font_scale)
            .h_metrics()
            .advance_width;

        let max_width = self.size.width;

        let bw = context.theme.style_for(&self.component_state).border_width as f32;
        let padding_x = 5.0 + bw;
        let padding_y = 5.0 + bw;

        let start_x = self.position.x as f32 + padding_x;
        let base_y =
            self.position.y as f32 + padding_y + v_metrics.ascent - self.visible_scrolling_offset;

        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0.0;

        for word in self.text.split_whitespace() {
            let word_width: f32 = word
                .chars()
                .map(|c| {
                    context
                        .font
                        .glyph(c)
                        .scaled(font_scale)
                        .h_metrics()
                        .advance_width
                })
                .sum();

            if current_width + word_width + (padding_x * 2.0) > max_width as f32 {
                let l = current_line.trim_end().to_string();
                lines.push(l);
                current_line = format!("{} ", word);
                current_width = word_width + space_width;
            } else {
                current_line.push_str(word);
                current_line.push(' ');
                current_width += word_width + space_width;
            }
        }

        if !current_line.trim().is_empty() {
            lines.push(current_line.trim_end().to_string());
        }

        let area_top = self.position.y as f32 + line_height + padding_y;
        let area_bottom =
            self.position.y as f32 + self.size.height as f32 - line_height - padding_y;

        for (i, line) in lines.iter().enumerate() {
            //
            // Trim lines...
            //
            let line_y = base_y + line_height * i as f32;
            if line_y + line_height < area_top || line_y > area_bottom {
                continue;
            }

            let glyphs: Vec<PositionedGlyph> = context
                .font
                .layout(
                    line,
                    font_scale,
                    point(start_x as f32, base_y + line_height * i as f32),
                )
                .collect();

            let text_color = context
                .theme
                .style_for(&self.component_state)
                .edit_text_color
                .value;

            for glyph in glyphs {
                if let Some(bb) = glyph.pixel_bounding_box() {
                    glyph.draw(|gx, gy, v| {
                        let x = gx as i32 + bb.min.x;
                        let y = gy as i32 + bb.min.y;
                        if x >= 0
                            && x < context.screen_size.width as i32
                            && y >= 0
                            && y < context.screen_size.height as i32
                        {
                            let idx = y as usize * context.screen_size.width as usize + x as usize;
                            if idx < context.buffer.len() {
                                context.buffer[idx] =
                                    TextArea::basic_aa(context.buffer[idx], text_color, v);
                            }
                        }
                    });
                }
            }
        }

        lines.iter().count()
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

    ///
    /// Draw the Button border.
    ///
    fn draw_border(&self, context: &mut BufferContext) {
        // let border_color = if self.focused {
        //     self.component_style_focused.border_color.value
        // } else {
        //     self.component_style.border_color.value
        // };
        // let bw = if self.focused {
        //     self.component_style_focused.border_width
        // } else {
        //     self.component_style.border_width
        // };
        let border_width = context.theme.style_for(&self.component_state).border_width;
        let border_color = context
            .theme
            .style_for(&self.component_state)
            .border_color
            .value;
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
                let is_top = y < y0 + border_width;
                let is_bottom = y >= y1.saturating_sub(border_width);
                let is_left = x < x0 + border_width;
                let is_right = x >= x1.saturating_sub(border_width);

                if is_top || is_bottom || is_left || is_right {
                    let idx = y * screen_width + x;
                    if idx < context.buffer.len() {
                        context.buffer[idx] = border_color;
                    }
                }
            }
        }
    }
}

impl Component for TextArea {
    fn handle_mouse_event(&mut self, input: MouseInput) {
        if self.component_state != ComponentState::Disabled {
            // println!(
            //     "self.dragging_scrollbar: {:?}, {:?}, {:?}, {:?}, {:?}",
            //     input,
            //     self.dragging_scrollbar,
            //     self.visible_scrolling_offset,
            //     input.position.y,
            //     self.last_mouse_y
            // );
            if input.pressed {
                if self.contains(input.position.x, input.position.y) {
                    self.focused = true;
                    self.component_state = ComponentState::Focused;
                } else {
                    self.focused = false;
                    self.component_state = ComponentState::Active;
                }
                self.dragging_scrollbar =
                    self.scrollbar_contains(input.position.x, input.position.y);
            }
            if self.dragging_scrollbar {
                // println!(
                //     "self.dragging_scrollbar: vso: {:?}, ipy: {:?}, lmy: {:?}",
                //     self.visible_scrolling_offset,
                //     input.position.y,
                //     self.last_mouse_y
                // );
                println!("scale_factor: {}", input.scale_factor);
                let delta = if let Some(last_y) = self.last_mouse_y {
                    last_y - (input.position.y * input.scale_factor)
                } else {
                    0.0
                };
                self.visible_scrolling_offset =
                    self.visible_scrolling_offset - delta as f32;
                self.last_mouse_y = Some(input.position.y);
            }
            //
            // TODO: BUG: If you click on another component but then release on TextArea it gets focus.
            //
            if input.just_released {
                if self.contains(input.position.x, input.position.y) {
                    self.focused = true;
                    self.component_state = ComponentState::Focused;
                } else {
                    self.focused = false;
                    self.component_state = ComponentState::Active;
                }
                //
                // Always:
                //
                self.dragging_scrollbar = false;
                self.last_mouse_y = None;
            }
        }
    }

    fn handle_mouse_wheel_event(
        &mut self,
        delta: &winit::event::MouseScrollDelta,
        _phase: &winit::event::TouchPhase,
    ) {
        if self.focused {
            match delta {
                winit::event::MouseScrollDelta::LineDelta(_x, y) => {
                    self.visible_scrolling_offset =
                        self.visible_scrolling_offset - (y * self.scroll_amount_y);
                }
                winit::event::MouseScrollDelta::PixelDelta(p) => {
                    self.visible_scrolling_offset = self.visible_scrolling_offset - p.x as f32
                }
            }
        }
    }

    fn handle_keyboard_event(&mut self, event: &winit::event::KeyEvent) {
        if self.focused && event.state == winit::event::ElementState::Pressed {
            match &event.logical_key {
                Key::Character(s) => {
                    self.insert_str(s);
                }
                Key::Named(named_key) => {
                    match named_key {
                        NamedKey::Space => {
                            self.insert_str(" ");
                        }
                        NamedKey::Enter => {
                            // TODO: Platform specific newline
                        }
                        NamedKey::Backspace => {
                            //
                            // TODO: Event to support undo/redo
                            //
                            let _char = self.text.pop();
                            self.cursor_index -= 1;
                            self.handle_event();
                        }
                        NamedKey::Delete => {
                            //
                            // TODO: Delete characters after the cursor
                            //
                        }
                        //
                        // TODO: Home, End, Up, Down, Left, Right
                        //
                        // TODO: SHIFT + Home, End, ...
                        //
                        _ => {}
                    }
                }
                Key::Unidentified(_) => {}
                Key::Dead(_) => {}
            }
        }
    }

    fn draw(&self, context: &mut BufferContext) {
        // let px = self.position.x as usize;
        // let py = self.position.y as usize;
        // let screen_w = context.screen_size.width as usize;
        // let screen_h = context.screen_size.height as usize;

        self.draw_background(context);
        let total_lines = self.draw_text(context);
        self.draw_border(context);

        // Scrollbars
        let font_scale = Scale::uniform(context.font_size);
        let v_metrics = context.font.v_metrics(font_scale);

        let line_height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap).ceil();
        let content_height = line_height * total_lines as f32;

        let area_x = self.position.x as usize;
        let area_y = self.position.y as usize;
        let area_w = self.size.width as usize;
        let area_h = self.size.height as usize;

        let track_x = area_x + area_w - self.scrollbar_width;
        let track_h = area_h;

        let visible_ratio = area_h as f32 / content_height;
        let thumb_height = (visible_ratio * area_h as f32).clamp(10.0, area_h as f32);
        let max_scroll = (content_height - area_h as f32).max(1.0);
        let thumb_y_offset =
            (self.visible_scrolling_offset / max_scroll) * (track_h as f32 - thumb_height);

        let back_color = context.theme.style_for(&self.component_state).back_color;
        let scrollbar_color_thumb = back_color.value;
        let scrollbar_color_track = back_color.lighten(ColorFactor::double()).value;

        // Draw scrollbar track
        for y in 0..track_h {
            for x in 0..self.scrollbar_width {
                let idx = (area_y + y) * context.screen_size.width as usize + (track_x + x);
                if idx < context.buffer.len() {
                    context.buffer[idx] = scrollbar_color_track; //0xFFE0E0E0;
                }
            }
        }

        //
        // Draw scrollbar thumb
        //
        let thumb_top = thumb_y_offset.round() as usize;
        for y in 0..(thumb_height as usize) {
            let ty = thumb_top + y;
            if ty >= track_h {
                break;
            }
            for x in 0..self.scrollbar_width {
                let idx = (area_y + ty) * context.screen_size.width as usize + (track_x + x);
                if idx < context.buffer.len() {
                    context.buffer[idx] = scrollbar_color_thumb; //0x00FF0000;
                }
            }
        }

        //
        // TODO: Draw cursor...
        //
        // TODO: Code is not wrapping. Need to move this to text drawing function?
        //
        if self.focused {
            // let cx = px + self.cursor_index * 6;
            // let cy = py + 10;
            // if cx < screen_w && cy < screen_h {
            //     context.buffer[cy * screen_w + cx] = 0x00FF0000; // red cursor
            // }
            // if cx < screen_w && cy < screen_h {
            //     context.buffer[cy * screen_w + cx + 1] = 0x00FF0000; // red cursor
            // }
            // if cx < screen_w && cy < screen_h {
            //     context.buffer[cy * screen_w + cx + 2] = 0x00FF0000; // red cursor
            // }
            // println!(
            //     "self.cursor_index = {} cx: {} cy: {} ",
            //     self.cursor_index, cx, cy
            // );
        }
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
