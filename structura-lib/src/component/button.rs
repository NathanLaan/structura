//!
//! Rust MVC-UI
//!

use crate::component::{ComponentState, ComponentStyle};
use crate::event::Callback;
use crate::geometry::Size;
use rusttype::{Font, Scale, point};
use std::ops::DerefMut;
use winit::event::ElementState;
//
// TODO: Button State: Idle, MouseOver, MouseDown
//

/*
   pub(crate) fn button_style_primary(
       theme: &Theme,
       status: Status,
   ) -> iced::widget::button::Style {
       let palette = theme.extended_palette();
       let background_color: Color = match status {
           Status::Active => palette.primary.strong.color,
           Status::Hovered => palette.primary.weak.color,
           Status::Pressed => palette.primary.base.color,
           Status::Disabled => palette.primary.weak.color,
       };
       iced::widget::button::Style {
           text_color: theme.palette().text,
           background: Some(background_color.into()),
           ..iced::widget::button::Style::default()
       }
   }
*/

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

pub struct Button {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub background_color: u32,
    pub border_color: u32,
    pub border_width: usize,
    pub text: String,
    pub component_state: ComponentState,
    pub component_style: ComponentStyle,
    pub on_clicked: Option<Callback<()>>,
    //pub was_clicked: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 200,
            height: 60,
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
            on_clicked: None,
            //was_clicked: false,
        }
    }
}

impl Button {
    pub fn contains(&self, px: usize, py: usize) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn update_state(&mut self, cursor_x: usize, cursor_y: usize, mouse_pressed: bool) {
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
    pub fn handle_mouse_event(&mut self, cursor_x: usize, cursor_y: usize, mouse_pressed: bool) {
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
    pub fn draw(
        &self,
        buffer: &mut [u32],
        screen_width: usize,
        screen_size: Size,
        font: &Font<'_>,
        font_size: f32,
    ) {
        self.fill_background(buffer, &screen_size);
        self.draw_border(buffer, &screen_size);

        // Text rendering parameters
        let font_scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(font_scale);

        let start_x = self.x as i32 + 10;
        let start_y = self.y as i32 + (self.height as i32 / 2) + (v_metrics.ascent / 2.0) as i32;

        let glyphs: Vec<_> = font
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
                        && x < screen_width as i32
                        && y >= 0
                        && (y as usize) < buffer.len() / screen_width
                    {
                        let idx = y as usize * screen_width + x as usize;
                        buffer[idx] = Self::basic_aa(buffer[idx], 0xFFFFFF, v);
                    }
                });
            }
        }
    }

    ///
    /// Fill in the background of the Button.
    ///
    fn fill_background(&self, buffer: &mut [u32], screen_size: &Size) {
        let screen_width = screen_size.width as usize;
        let screen_height = screen_size.height as usize;
        let bw = self.border_width;
        let x0 = self.x;
        let y0 = self.y;
        let x1 = self.x + self.width;
        let y1 = self.y + self.height;
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
                if idx < buffer.len() {
                    buffer[idx] = background_color;
                }
            }
        }
    }

    ///
    /// Draw the Button border.
    ///
    fn draw_border(&self, buffer: &mut [u32], screen_size: &Size) {
        let bw = self.border_width;
        let x0 = self.x;
        let y0 = self.y;
        let x1 = self.x + self.width;
        let y1 = self.y + self.height;
        let screen_width = screen_size.width as usize;
        let screen_height = screen_size.height as usize;
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
                    if idx < buffer.len() {
                        buffer[idx] = self.border_color;
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
}
