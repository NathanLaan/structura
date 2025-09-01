use winit::event::{ElementState, MouseScrollDelta, VirtualKeyCode, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::component::Component;
use crate::event::MouseInput;

use crate::geometry::{Point, Size};
use crate::view::BufferContext;



pub struct TextEdit {
    // Position and size
    position: Point,
    size: Size,

    // Text content and cursor
    text: String,
    cursor_position: usize,

    // Selection state
    selection_start: Option<usize>,
    selection_end: Option<usize>,

    // Visual properties
    font_size: f64,
    line_height: f64,
    padding: f64,

    // State
    is_focused: bool,
    cursor_visible: bool,
    cursor_blink_timer: f64,

    // Scrolling
    scroll_offset_y: f64,

    // Colors (as RGB packed into u32)
    background_color: u32,
    text_color: u32,
    cursor_color: u32,
    selection_color: u32,
    border_color: u32,
}

impl TextEdit {
    pub fn new() -> Self {
        Self {
            position: Point{x:0.0, y:0.0},
            size: Size{width:200, height: 100},
            text: String::new(),
            cursor_position: 0,
            selection_start: None,
            selection_end: None,
            font_size: 14.0,
            line_height: 16.0,
            padding: 4.0,
            is_focused: false,
            cursor_visible: true,
            cursor_blink_timer: 0.0,
            scroll_offset_y: 0.0,
            background_color: 0xFFFFFF, // White
            text_color: 0x000000,       // Black
            cursor_color: 0x000000,     // Black
            selection_color: 0x3399FF,  // Blue
            border_color: 0xCCCCCC,     // Light gray
        }
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.cursor_position = text.len();
        self.text = text;
        self
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
        if focused {
            self.cursor_visible = true;
            self.cursor_blink_timer = 0.0;
        }
    }

    fn char_to_cursor_pos(&self, x: f64, y: f64) -> usize {
        // Simple approximation - in a real implementation you'd use proper text metrics
        let relative_x = x - self.position.x - self.padding;
        let relative_y = y - self.position.y - self.padding + self.scroll_offset_y;

        if relative_y < 0.0 {
            return 0;
        }

        let line_index = (relative_y / self.line_height).floor() as usize;
        let lines: Vec<&str> = self.text.lines().collect();

        if line_index >= lines.len() {
            return self.text.len();
        }

        // Simple character width approximation (monospace assumption)
        let char_width = self.font_size * 0.6; // Approximate character width
        let char_index_in_line = (relative_x / char_width).round() as usize;

        let mut position = 0;
        for (i, line) in lines.iter().enumerate() {
            if i == line_index {
                return position + char_index_in_line.min(line.len());
            }
            position += line.len() + 1; // +1 for newline
        }

        self.text.len()
    }

    fn cursor_to_screen_pos(&self, cursor_pos: usize) -> Point {
        let lines: Vec<&str> = self.text.lines().collect();
        let mut current_pos = 0;
        let char_width = self.font_size * 0.6;

        for (line_idx, line) in lines.iter().enumerate() {
            let line_end = current_pos + line.len();

            if cursor_pos <= line_end {
                let char_in_line = cursor_pos - current_pos;
                let x = self.position.x + self.padding + (char_in_line as f64 * char_width);
                let y = self.position.y + self.padding + (line_idx as f64 * self.line_height) - self.scroll_offset_y;
                return Point { x, y };
            }

            current_pos = line_end + 1; // +1 for newline
        }

        // Cursor at end of text
        let last_line_idx = lines.len().saturating_sub(1);
        let x = self.position.x + self.padding + (lines.last().unwrap_or(&"").len() as f64 * char_width);
        let y = self.position.y + self.padding + (last_line_idx as f64 * self.line_height) - self.scroll_offset_y;
        Point { x, y }
    }

    fn insert_char(&mut self, ch: char) {
        if let Some(_) = self.selection_start {
            self.delete_selection();
        }

        self.text.insert(self.cursor_position, ch);
        self.cursor_position += ch.len_utf8();
        self.clear_selection();
    }

    fn delete_char_before(&mut self) {
        if let Some(_) = self.selection_start {
            self.delete_selection();
            return;
        }

        if self.cursor_position > 0 {
            let mut char_start = self.cursor_position - 1;
            while char_start > 0 && !self.text.is_char_boundary(char_start) {
                char_start -= 1;
            }
            self.text.remove(char_start);
            self.cursor_position = char_start;
        }
    }

    fn delete_char_after(&mut self) {
        if let Some(_) = self.selection_start {
            self.delete_selection();
            return;
        }

        if self.cursor_position < self.text.len() {
            let mut char_end = self.cursor_position + 1;
            while char_end < self.text.len() && !self.text.is_char_boundary(char_end) {
                char_end += 1;
            }
            self.text.drain(self.cursor_position..char_end);
        }
    }

    fn move_cursor_left(&mut self, shift_held: bool) {
        if !shift_held && self.selection_start.is_some() {
            self.cursor_position = self.selection_start.unwrap().min(self.selection_end.unwrap());
            self.clear_selection();
            return;
        }

        if shift_held && self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_position);
        }

        if self.cursor_position > 0 {
            let mut new_pos = self.cursor_position - 1;
            while new_pos > 0 && !self.text.is_char_boundary(new_pos) {
                new_pos -= 1;
            }
            self.cursor_position = new_pos;
        }

        if shift_held {
            self.selection_end = Some(self.cursor_position);
        } else {
            self.clear_selection();
        }
    }

    fn move_cursor_right(&mut self, shift_held: bool) {
        if !shift_held && self.selection_start.is_some() {
            self.cursor_position = self.selection_start.unwrap().max(self.selection_end.unwrap());
            self.clear_selection();
            return;
        }

        if shift_held && self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_position);
        }

        if self.cursor_position < self.text.len() {
            let mut new_pos = self.cursor_position + 1;
            while new_pos < self.text.len() && !self.text.is_char_boundary(new_pos) {
                new_pos += 1;
            }
            self.cursor_position = new_pos;
        }

        if shift_held {
            self.selection_end = Some(self.cursor_position);
        } else {
            self.clear_selection();
        }
    }

    fn move_cursor_up(&mut self, shift_held: bool) {
        if shift_held && self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_position);
        }

        let current_pos = self.cursor_to_screen_pos(self.cursor_position);
        let new_y = current_pos.y - self.line_height;
        let new_pos = self.char_to_cursor_pos(current_pos.x, new_y);
        self.cursor_position = new_pos;

        if shift_held {
            self.selection_end = Some(self.cursor_position);
        } else {
            self.clear_selection();
        }
    }

    fn move_cursor_down(&mut self, shift_held: bool) {
        if shift_held && self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_position);
        }

        let current_pos = self.cursor_to_screen_pos(self.cursor_position);
        let new_y = current_pos.y + self.line_height;
        let new_pos = self.char_to_cursor_pos(current_pos.x, new_y);
        self.cursor_position = new_pos;

        if shift_held {
            self.selection_end = Some(self.cursor_position);
        } else {
            self.clear_selection();
        }
    }

    fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }

    fn delete_selection(&mut self) {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let start_pos = start.min(end);
            let end_pos = start.max(end);
            self.text.drain(start_pos..end_pos);
            self.cursor_position = start_pos;
            self.clear_selection();
        }
    }

    fn is_point_inside(&self, x: f64, y: f64) -> bool {
        x >= self.position.x && x <= self.position.x + self.size.width as f64 &&
            y >= self.position.y && y <= self.position.y + self.size.height as f64
    }

    fn ensure_cursor_visible(&mut self) {
        let cursor_screen_pos = self.cursor_to_screen_pos(self.cursor_position);
        let visible_top = self.position.y + self.padding;
        let visible_bottom = self.position.y + self.size.height as f64 - self.padding;

        if cursor_screen_pos.y < visible_top {
            self.scroll_offset_y += visible_top - cursor_screen_pos.y;
        } else if cursor_screen_pos.y + self.line_height > visible_bottom {
            self.scroll_offset_y -= (cursor_screen_pos.y + self.line_height) - visible_bottom;
        }
    }
}

impl Component for TextEdit {
    fn handle_mouse_event(&mut self, input: MouseInput) {
        if input.pressed {
            //if input.button == MouseButton::Left {
                if self.is_point_inside(input.position.x, input.position.y) {
                    self.set_focused(true);
                    let new_cursor_pos = self.char_to_cursor_pos(input.position.x, input.position.y);

                    // Start selection
                    self.selection_start = Some(new_cursor_pos);
                    self.cursor_position = new_cursor_pos;
                    self.selection_end = Some(new_cursor_pos);
                } else {
                    self.set_focused(false);
                    self.clear_selection();
                }
            //}
        }else{
        //if input.button == MouseButton::Left && self.is_focused {
                // End selection - if start and end are the same, clear selection
                if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
                    if start == end {
                        self.clear_selection();
                    }
                }
        //    }
        }
    }

    fn handle_mouse_wheel_event(&mut self, event: &MouseScrollDelta, _phase: &TouchPhase) {
        if !self.is_focused {
            return;
        }

        match event {
            MouseScrollDelta::LineDelta(_x, y) => {
                self.scroll_offset_y -= y * self.line_height * 3.0; // Scroll 3 lines at a time
            }
            MouseScrollDelta::PixelDelta(delta) => {
                self.scroll_offset_y -= delta.y;
            }
        }

        // Clamp scroll offset
        self.scroll_offset_y = self.scroll_offset_y.max(0.0);
    }

    fn handle_keyboard_event(&mut self, event: &KeyEvent) {
        if !self.is_focused || event.state != ElementState::Pressed {
            return;
        }

        //
        // TODO: Modifiers
        //
        //let shift_held = event.modifiers.shift_key();
        //let ctrl_held = event.modifiers.control_key();

        let shift_held = event.modifiers.shift();

        match event.physical_key {
            PhysicalKey::Code(KeyCode::Backspace) => {
                self.delete_char_before();
                self.ensure_cursor_visible();
            }
            PhysicalKey::Code(KeyCode::Delete) => {
                self.delete_char_after();
                self.ensure_cursor_visible();
            }
            PhysicalKey::Code(KeyCode::ArrowLeft) => {
                self.move_cursor_left(shift_held);
                self.ensure_cursor_visible();
            }
            PhysicalKey::Code(KeyCode::ArrowRight) => {
                self.move_cursor_right(shift_held);
                self.ensure_cursor_visible();
            }
            PhysicalKey::Code(KeyCode::ArrowUp) => {
                self.move_cursor_up(shift_held);
                self.ensure_cursor_visible();
            }
            PhysicalKey::Code(KeyCode::ArrowDown) => {
                self.move_cursor_down(shift_held);
                self.ensure_cursor_visible();
            }
            PhysicalKey::Code(KeyCode::Home) => {
                if shift_held && self.selection_start.is_none() {
                    self.selection_start = Some(self.cursor_position);
                }
                self.cursor_position = 0;
                if shift_held {
                    self.selection_end = Some(self.cursor_position);
                } else {
                    self.clear_selection();
                }
                self.ensure_cursor_visible();
            }
            PhysicalKey::Code(KeyCode::End) => {
                if shift_held && self.selection_start.is_none() {
                    self.selection_start = Some(self.cursor_position);
                }
                self.cursor_position = self.text.len();
                if shift_held {
                    self.selection_end = Some(self.cursor_position);
                } else {
                    self.clear_selection();
                }
                self.ensure_cursor_visible();
            }
            PhysicalKey::Code(KeyCode::Enter) => {
                self.insert_char('\n');
                self.ensure_cursor_visible();
            }
            PhysicalKey::Code(KeyCode::Tab) => {
                self.insert_char('\t');
                self.ensure_cursor_visible();
            }
            _ => {
                // Handle text input
                if let Some(text) = &event.text {
                    for ch in text.chars() {
                        if ch.is_control() {
                            continue;
                        }
                        self.insert_char(ch);
                    }
                    self.ensure_cursor_visible();
                }
            }
        }
    }

    fn draw(&self, context: &mut BufferContext) {
        // Draw background
        context.draw_rect(
            self.position.x,
            self.position.y,
            self.size.width as f64,
            self.size.height as f64,
            self.background_color,
        );

        // Draw border
        context.draw_rect(
            self.position.x,
            self.position.y,
            self.size.width as f64,
            self.size.height as f64,
            self.border_color,
        );

        // Set up clipping region for text
        let text_area_x = self.position.x + self.padding;
        let text_area_y = self.position.y + self.padding;
        let text_area_width = self.size.width as f64 - 2.0 * self.padding;
        let text_area_height = self.size.height as f64 - 2.0 * self.padding;

        // Draw selection background
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let start_pos = start.min(end);
            let end_pos = start.max(end);

            // Simplified selection rendering
            // TODO: need to handle multi-line selections
            let start_screen = self.cursor_to_screen_pos(start_pos);
            let end_screen = self.cursor_to_screen_pos(end_pos);

            if start_screen.y == end_screen.y {
                // Single line selection
                context.draw_rect(
                    start_screen.x,
                    start_screen.y,
                    end_screen.x - start_screen.x,
                    self.line_height,
                    self.selection_color,
                );
            }
        }

        // Draw text
        let lines: Vec<&str> = self.text.lines().collect();
        for (line_idx, line) in lines.iter().enumerate() {
            let y = text_area_y + (line_idx as f64 * self.line_height) - self.scroll_offset_y;

            // Only draw lines that are visible
            if y + self.line_height >= text_area_y && y <= text_area_y + text_area_height {
                context.draw_text(line, text_area_x, y, self.text_color);
            }
        }

        // Draw cursor
        if self.is_focused && self.cursor_visible {
            let cursor_pos = self.cursor_to_screen_pos(self.cursor_position);
            context.draw_line(
                cursor_pos.x,
                cursor_pos.y,
                cursor_pos.x,
                cursor_pos.y + self.line_height,
                self.cursor_color,
            );
        }
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.position = Point::new(x, y);
    }

    fn get_position(&self) -> Point {
        self.position
    }

    fn set_size(&mut self, width: usize, height: usize) {
        self.size = Size::new(width, height);
    }

    fn get_size(&self) -> Size {
        self.size.clone()
    }
}