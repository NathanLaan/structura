//!
//! **Structura**
//!
//! Colors are defined in `ARGB (0xAARRGGBB)` format as `u32` type.
//!

use crate::component::ComponentState;

///
/// The factor to be applied for `Color::lighten()` and `Color::darken()`.
///
/// The `ColorFactor` must be a number between `0.0` and `1.0`.
///
pub struct ColorFactor {
    pub factor: f32,
}

impl ColorFactor {
    pub fn new(factor: f32) -> ColorFactor {
        ColorFactor { factor }
    }
    pub fn double() -> ColorFactor {
        ColorFactor {
            factor: ColorFactor::default().factor * 2.0,
        }
    }
}

impl Default for ColorFactor {
    fn default() -> Self {
        ColorFactor { factor: 0.2 }
    }
}

///
/// A `Color`, used by `Components` to draw controls, text, and other GUI items.
///
/// `Color` is defined using tbe `ARGB (0xAARRGGBB)` format, using a `u32` type.
///
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
    pub fn darken(&self, color_factor: ColorFactor) -> Color {
        Color::adjust_color_brightness(&self, -color_factor.factor)
    }

    ///
    /// Lighten the color by extracting each channel, and adjusting each channel by an
    /// internal fixed `delta` factor.
    ///
    pub fn lighten(&self, color_factor: ColorFactor) -> Color {
        Color::adjust_color_brightness(&self, color_factor.factor)
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

///
/// The default `ComponentTheme`.
///
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
            ComponentState::Hovered => ComponentStyle::STYLE_ACTIVE.lighten(),
            ComponentState::Pressed => ComponentStyle::STYLE_PRESSED.darken(),
            ComponentState::Focused => ComponentStyle::STYLE_FOCUSED,
            ComponentState::Disabled => ComponentStyle::STYLE_DISABLED,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComponentStyle {
    /// Foreground text color in a `Component` where text can be edited.
    pub edit_text_color: Color,

    /// Background color in a `Component` where text can be edited.
    pub edit_back_color: Color,

    /// Foreground color. Used for text on components, such as a `Button`.
    pub fore_color: Color,
    pub back_color: Color,
    pub cursor_color: Color,
    pub border_color: Color,
    pub border_width: usize,
}

impl ComponentStyle {
    ///
    /// Constructor.
    ///
    pub const fn new(
        text_color: u32,
        fore_color: u32,
        back_color: u32,
        cursor_color: u32,
        border_color: u32,
        border_width: usize,
    ) -> Self {
        Self {
            edit_text_color: Color { value: text_color },
            edit_back_color: Color { value: back_color },
            // TODO: edit_border_color...
            fore_color: Color { value: fore_color },
            back_color: Color { value: back_color },
            cursor_color: Color {
                value: cursor_color,
            },
            border_color: Color {
                value: border_color,
            },
            border_width,
        }
    }

    pub fn darken(&self) -> ComponentStyle {
        ComponentStyle {
            edit_text_color: self.edit_text_color.darken(ColorFactor::default()),
            edit_back_color: self.edit_back_color.darken(ColorFactor::default()),
            fore_color: self.fore_color.darken(ColorFactor::default()),
            back_color: self.back_color.darken(ColorFactor::default()),
            cursor_color: self.cursor_color.darken(ColorFactor::default()),
            border_color: self.border_color.darken(ColorFactor::default()),
            border_width: self.border_width,
        }
    }

    pub fn lighten(&self) -> ComponentStyle {
        ComponentStyle {
            edit_text_color: self.edit_text_color.lighten(ColorFactor::default()),
            edit_back_color: self.edit_back_color.lighten(ColorFactor::default()),
            fore_color: self.fore_color.lighten(ColorFactor::default()),
            back_color: self.back_color.lighten(ColorFactor::default()),
            cursor_color: self.cursor_color.lighten(ColorFactor::default()),
            border_color: self.border_color.lighten(ColorFactor::default()),
            border_width: self.border_width,
        }
    }

    pub const STYLE_ACTIVE: ComponentStyle = ComponentStyle {
        edit_text_color: Color { value: 0xFF222222 },
        edit_back_color: Color { value: 0xFFFFFFFF },
        fore_color: Color { value: 0xFFFF3333 },
        back_color: Color { value: 0xFF0033CC },
        cursor_color: Color { value: 0xFF000000 },
        border_color: Color { value: 0xFF000000 },
        border_width: 2,
    };

    pub const STYLE_HOVERED: ComponentStyle = ComponentStyle {
        edit_text_color: Color { value: 0xFF000000 },
        edit_back_color: Color { value: 0xFFFFFFFF },
        fore_color: Color { value: 0xFFFF0000 },
        back_color: Color { value: 0xFF0077CC },
        cursor_color: Color { value: 0xFF000000 },
        border_color: Color { value: 0xFF000000 },
        border_width: 2,
    };

    pub const STYLE_PRESSED: ComponentStyle = ComponentStyle {
        edit_text_color: Color { value: 0xFF000000 },
        edit_back_color: Color { value: 0xFFFFFFFF },
        fore_color: Color { value: 0xFF000000 },
        back_color: Color { value: 0xFF0099CC },
        cursor_color: Color { value: 0xFF000000 },
        border_color: Color { value: 0xFF0077CC },
        border_width: 2,
    };

    pub const STYLE_FOCUSED: ComponentStyle = ComponentStyle {
        edit_text_color: Color { value: 0xFF000000 },
        edit_back_color: Color { value: 0xFFFFFFFF },
        fore_color: Color { value: 0xFF000000 },
        back_color: Color { value: 0xFF0099CC },
        cursor_color: Color { value: 0xFF000000 },
        border_color: Color { value: 0xFF000000 },
        border_width: 2,
    };

    pub const STYLE_DISABLED: ComponentStyle = ComponentStyle {
        edit_text_color: Color { value: 0xFF000000 },
        edit_back_color: Color { value: 0xFFFFFFFF },
        fore_color: Color { value: 0xFF000000 },
        back_color: Color { value: 0xFFCCCCCC },
        cursor_color: Color { value: 0xFF000000 },
        border_color: Color { value: 0xFF000000 },
        border_width: 2,
    };

    pub fn default_for(state: &ComponentState) -> Self {
        match state {
            ComponentState::Active => ComponentStyle::STYLE_ACTIVE.darken(),
            ComponentState::Hovered => ComponentStyle::STYLE_ACTIVE,
            ComponentState::Pressed => ComponentStyle::STYLE_ACTIVE.lighten(),
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
