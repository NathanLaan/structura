//!
//! **Structura**
//!
//! Colors are defined in `ARGB (0xAARRGGBB)` format as `u32` type.
//!

use crate::component::ComponentState;

#[derive(Debug, Clone)]
pub struct Color {
    pub value: u32,
}

impl Color {
    ///
    /// Constructor.
    ///
    pub fn new(value: u32) -> Self {
        Color { value }
    }

    ///
    /// Darken the color by extracting each channel, and adjusting each channel by an
    /// internal fixed `delta` factor.
    ///
    pub fn darken(&self) -> Color {
        Color::adjust_color_brightness(&self, -0.2)
    }

    ///
    /// Lighten the color by extracting each channel, and adjusting each channel by an
    /// internal fixed `delta` factor.
    ///
    pub fn lighten(&self) -> Color {
        Color::adjust_color_brightness(&self, 0.2)
    }

    ///
    /// Adjust the color by extracting each channel, and adjusting each channel by the
    /// specified `delta` factor, which must be between -1.0 and +1.0.
    ///
    /// TODO: Bounds checking on `delta`.
    ///
    fn adjust_color_brightness(color: &Color, delta: f32) -> Color {
        let adjusted_value = Color::adjust_color_value_brightness(color.value, delta);
        Color {
            value: adjusted_value,
        }
    }

    ///
    /// Adjust the color by extracting each channel, and adjusting each channel by the
    /// specified `delta` factor, which must be between -1.0 and +1.0.
    ///
    /// TODO: Bounds checking on `delta`.
    ///
    fn adjust_color_value_brightness(color: u32, delta: f32) -> u32 {
        let a = (color >> 24) & 0xFF;
        let r = ((color >> 16) & 0xFF) as f32;
        let g = ((color >> 8) & 0xFF) as f32;
        let b = (color & 0xFF) as f32;
        let scale = if delta < 0.0 {
            1.0 + delta // darken
        } else {
            1.0 - delta
        };
        let offset = if delta < 0.0 { 0.0 } else { 255.0 * delta };
        let new_red = (r * scale + offset).clamp(0.0, 255.0) as u32;
        let new_green = (g * scale + offset).clamp(0.0, 255.0) as u32;
        let new_blue = (b * scale + offset).clamp(0.0, 255.0) as u32;
        (a << 24) | (new_red << 16) | (new_green << 8) | new_blue
    }
}

///
/// A Theme consists of a mapping between each possible `ComponentState`
///
pub trait ComponentTheme {
    fn style_for(&self, state: &ComponentState) -> ComponentStyle;
}

pub struct DefaultComponentTheme;

impl Default for DefaultComponentTheme {
    fn default() -> Self {
        DefaultComponentTheme
    }
}

impl ComponentTheme for DefaultComponentTheme {
    fn style_for(&self, state: &ComponentState) -> ComponentStyle {
        match state {
            ComponentState::Active => ComponentStyle::STYLE_ACTIVE,
            ComponentState::Hovered => ComponentStyle::STYLE_ACTIVE,
            ComponentState::Pressed => ComponentStyle::STYLE_PRESSED,
            ComponentState::Focused => ComponentStyle::STYLE_FOCUSED,
            ComponentState::Disabled => ComponentStyle::STYLE_DISABLED,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComponentStyle {
    pub text: Color,
    pub back_color: u32,
    pub cursor_color: u32,
    pub border_color: u32,
    pub border_width: usize,
}

impl ComponentStyle {
    ///
    /// Constructor.
    ///
    pub const fn new(
        text_color: u32,
        back_color: u32,
        cursor_color: u32,
        border_color: u32,
        border_width: usize,
    ) -> Self {
        Self {
            text: Color { value: text_color },
            back_color,
            cursor_color,
            border_color,
            border_width,
        }
    }

    pub fn darken(&self) -> ComponentStyle {
        ComponentStyle {
            text: self.text.darken(),
            back_color: self.back_color,
            cursor_color: self.cursor_color,
            border_color: self.border_color,
            border_width: self.border_width,
        }
    }

    pub fn lighten(&self) -> ComponentStyle {
        ComponentStyle {
            text: self.text.lighten(),
            back_color: self.back_color,
            cursor_color: self.cursor_color,
            border_color: self.border_color,
            border_width: self.border_width,
        }
    }

    pub const STYLE_ACTIVE: ComponentStyle = ComponentStyle {
        text: Color { value: 0xFF222222 },
        //text_color: 0xFF000000,
        back_color: 0x0033CC,
        cursor_color: 0x000000,
        border_color: 0x000000,
        border_width: 2,
    };

    pub const STYLE_HOVERED: ComponentStyle = ComponentStyle {
        text: Color { value: 0xFF000000 },
        //text_color: 0xFF000000,
        back_color: 0x0077CC,
        cursor_color: 0x000000,
        border_color: 0x000000,
        border_width: 2,
    };

    pub const STYLE_PRESSED: ComponentStyle = ComponentStyle {
        text: Color { value: 0xFF000000 },
        //text_color: 0xFF000000,
        back_color: 0x0099CC,
        cursor_color: 0x000000,
        border_color: 0x000000,
        border_width: 2,
    };

    pub const STYLE_FOCUSED: ComponentStyle = ComponentStyle {
        text: Color { value: 0xFF000000 },
        //text_color: 0xFF000000,
        back_color: 0x0099CC,
        cursor_color: 0x000000,
        border_color: 0x000000,
        border_width: 2,
        // back_color: 0x0033CC,
        // text_color: 0xCCCCCC,
        // cursor_color: 0xCCCCCC,
        // border_color: 0xFF3333,
        // border_width: 3,
    };

    pub const STYLE_DISABLED: ComponentStyle = ComponentStyle {
        text: Color { value: 0xFF000000 },
        //text_color: 0xFF000000,
        back_color: 0xCCCCCC,
        cursor_color: 0x000000,
        border_color: 0x000000,
        border_width: 2,
    };

    pub fn default_for(state: &ComponentState) -> Self {
        match state {
            ComponentState::Active => ComponentStyle::STYLE_ACTIVE,
            ComponentState::Hovered => ComponentStyle::STYLE_ACTIVE.darken(),
            ComponentState::Pressed => ComponentStyle::STYLE_PRESSED,
            ComponentState::Focused => ComponentStyle::STYLE_FOCUSED,
            ComponentState::Disabled => ComponentStyle::STYLE_DISABLED,
        }
    }
}

impl Default for ComponentStyle {
    fn default() -> Self {
        ComponentStyle::STYLE_ACTIVE
    }
}
