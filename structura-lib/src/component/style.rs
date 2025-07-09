//!
//!
//!

use crate::component::ComponentState;

#[derive(Debug, Clone)]
pub struct ComponentStyle {
    pub text_color: u32,
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
            text_color,
            back_color,
            cursor_color,
            border_color,
            border_width,
        }
    }

    pub const STYLE_ACTIVE: ComponentStyle = ComponentStyle {
        text_color: 0x000000,
        back_color: 0x0033CC,
        cursor_color: 0x000000,
        border_color: 0x000000,
        border_width: 2,
    };

    pub const STYLE_HOVERED: ComponentStyle = ComponentStyle {
        text_color: 0x000000,
        back_color: 0x0077CC,
        cursor_color: 0x000000,
        border_color: 0x000000,
        border_width: 2,
    };

    pub const STYLE_PRESSED: ComponentStyle = ComponentStyle {
        text_color: 0x000000,
        back_color: 0x0099CC,
        cursor_color: 0x000000,
        border_color: 0x000000,
        border_width: 2,
    };

    pub const STYLE_FOCUSED: ComponentStyle = ComponentStyle {
        text_color: 0x000000,
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
        text_color: 0x000000,
        back_color: 0xCCCCCC,
        cursor_color: 0x000000,
        border_color: 0x000000,
        border_width: 2,
    };

    pub fn default_for(state: &ComponentState) -> Self {
        match state {
            ComponentState::Active => ComponentStyle::STYLE_ACTIVE,
            ComponentState::Hovered => ComponentStyle::STYLE_HOVERED,
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