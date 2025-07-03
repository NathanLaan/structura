//!
//! # Structura Component: TextArea.
//!
//! A basic editable, multiline text `Component`.
//!

use crate::component::{Component, ComponentStyle};
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
#[derive(Debug, Clone)]
pub struct TextArea {
    pub text: String,
    pub cursor_index: usize,
    pub position: Point,
    pub size: Size,
    pub focused: bool,
    component_style: ComponentStyle,
    component_style_focused: ComponentStyle,
    visible_scrolling_offset: f32,
    pub dragging_scrollbar: bool,
    pub last_mouse_y: f64,
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
            component_style: ComponentStyle {
                back_color: 0x0033CC,
                text_color: 0x000000,
                cursor_color: 0x000000,
                border_color: 0x000000,
                border_width: 1,
            },
            component_style_focused: ComponentStyle {
                back_color: 0x0033CC,
                text_color: 0xCCCCCC,
                cursor_color: 0xCCCCCC,
                border_color: 0xFF3333,
                border_width: 3,
            },
            visible_scrolling_offset: 0.0,
            dragging_scrollbar: false,
            last_mouse_y: 0.0,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.position.x
            && px < self.position.x + self.size.width as f64
            && py >= self.position.y
            && py < self.position.y + self.size.height as f64
    }

    fn is_scrollbar_hit(&self, x: f64, y: f64) -> bool {
        let scroll_x = self.position.x + (self.size.width - 6) as f64;
        x >= scroll_x
            && x <= scroll_x + 6.0
            && y >= self.position.y
            && y <= self.position.y + self.size.height as f64
    }

    fn draw_background(&self, context: &mut BufferContext) {
        let px = self.position.x as usize;
        let py = self.position.y as usize;
        let w = self.size.width;
        let h = self.size.height;
        let screen_w = context.screen_size.width as usize;
        let screen_h = context.screen_size.height as usize;

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

        let padding_x = if self.focused {
            self.component_style_focused.border_width as f32 + 5.0
        } else {
            self.component_style_focused.border_width as f32 + 5.0
        };
        let padding_y = if self.focused {
            self.component_style_focused.border_width as f32 + 5.0
        } else {
            self.component_style_focused.border_width as f32 + 5.0
        };

        let start_x = self.position.x as f32 + padding_x;
        let start_y =
            self.position.y + (self.size.height as f64 / 2.0) + (v_metrics.ascent / 2.0) as f64;

        //let base_y = self.position.y as f32 + padding_y + v_metrics.ascent;
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
                                    TextArea::basic_aa(context.buffer[idx], 0x000000, v);
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
        let border_color = if self.focused {
            self.component_style_focused.border_color
        } else {
            self.component_style.border_color
        };
        let bw = if self.focused {
            self.component_style_focused.border_width
        } else {
            self.component_style.border_width
        };
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
                        context.buffer[idx] = border_color;
                    }
                }
            }
        }
    }
}

impl Component for TextArea {
    fn handle_mouse_event(&mut self, input: MouseInput) {
        if input.pressed {
            self.focused = self.contains(input.position.x, input.position.y);
        }
    }

    fn handle_mouse_wheel_event(
        &mut self,
        delta: &winit::event::MouseScrollDelta,
        phase: &winit::event::TouchPhase,
    ) {
        match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                self.visible_scrolling_offset = (self.visible_scrolling_offset - y);
            }
            winit::event::MouseScrollDelta::PixelDelta(p) => {
                self.visible_scrolling_offset = (self.visible_scrolling_offset - p.x as f32);
            }
        }
    }

    fn handle_keyboard_event(&mut self, event: &winit::event::KeyEvent) {
        if self.focused && event.state == winit::event::ElementState::Pressed {
            match &event.logical_key {
                Key::Character(s) => {
                    self.text.push_str(s);
                }
                Key::Named(named_key) => {
                    match named_key {
                        NamedKey::Space => {
                            self.text.push_str(" ");
                        }
                        NamedKey::Enter => {
                            // TODO: Platform specific newline
                        }
                        NamedKey::Backspace => {
                            //
                            // TODO: Delete at the cursor!
                            //
                            // TODO: Event to support undo/redo
                            //
                            let _char = self.text.pop();
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
        let px = self.position.x as usize;
        let py = self.position.y as usize;
        let w = self.size.width;
        let h = self.size.height;
        let screen_w = context.screen_size.width as usize;
        let screen_h = context.screen_size.height as usize;

        self.draw_background(context);
        self.draw_border(context);
        let total_lines = self.draw_text(context);

        // Scrollbars
        let font_scale = Scale::uniform(context.font_size);
        let v_metrics = context.font.v_metrics(font_scale);
        let line_height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap).ceil();
        let content_height = line_height * total_lines as f32;

        let line_height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap).ceil();
        let content_height = line_height * total_lines as f32;

        let area_x = self.position.x as usize;
        let area_y = self.position.y as usize;
        let area_w = self.size.width as usize;
        let area_h = self.size.height as usize;

        let scrollbar_width = 20;
        let track_x = area_x + area_w - scrollbar_width;
        let track_h = area_h;

        let visible_ratio = area_h as f32 / content_height;
        let thumb_height = (visible_ratio * area_h as f32).clamp(10.0, area_h as f32);
        let max_scroll = (content_height - area_h as f32).max(1.0);
        let thumb_y_offset =
            (self.visible_scrolling_offset / max_scroll) * (track_h as f32 - thumb_height);

        // draw track
        for y in 0..track_h {
            for x in 0..scrollbar_width {
                let idx = (area_y + y) * context.screen_size.width as usize + (track_x + x);
                if idx < context.buffer.len() {
                    context.buffer[idx] = 0xFFE0E0E0;
                }
            }
        }

        // draw thumb
        let thumb_top = thumb_y_offset.round() as usize;
        for y in 0..(thumb_height as usize) {
            let ty = thumb_top + y;
            if ty >= track_h {
                break;
            }
            for x in 0..scrollbar_width {
                let idx = (area_y + ty) * context.screen_size.width as usize + (track_x + x);
                if idx < context.buffer.len() {
                    context.buffer[idx] = 0x00FF0000;
                }
            }
        }

        //
        // TODO: Draw cursor...
        //
        if self.focused {
            let cx = px + self.cursor_index * 6;
            let cy = py + 10;
            if cx < screen_w && cy < screen_h {
                context.buffer[cy * screen_w + cx] = 0xFFFF0000; // red cursor
            }
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
