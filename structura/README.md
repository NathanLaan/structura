# Structura-Lib Rust GUI Framework

**Structura** is a Rust GUI framework created to learn how to create a GUI framework in Rust. Maybe one day it will be "OK enough" to actually use for something, but you really should not use this.

The **Structura** GUI framework is based on cross-platform components, but has only been tested on Linux under Wayland.

## Dependencies

**Structura** is based on the following Rust libraries:

- [winit](https://docs.rs/winit/latest/winit/) cross-platform window management library.
- [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/) 2D buffer library.
- [rusttype](https://docs.rs/rusttype/latest/rusttype/) font library.
- [Tokio](https://docs.rs/tokio/latest/tokio/), an event-driven, non-blocking I/O platform for writing asynchronous applications.

## Architecture

**Structura** is loosely designed around a Model-View-Controller (MVC) architecture.

The **Structura** UI components are designed around a fluent API where possible. For example:
```rust
let button1 = Button::default()
    .set_text("Button 1!".to_string())
    .on_click(|| {
        println!("button1.on_click()");
    });
```

The minimum viable **Structura** app looks like this:
```rust
use structura::app::Application;
use structura::component::button::Button;
use structura::container::panel::Panel;

fn main() {
    let mut panel = Panel::new();
    panel.push(Box::new(Button::default()));
    let mut application = Application::new(Box::new(panel));
    application.run();
}
```

### Traits

- `Component`: Displays output to users and/or allows users to interact. Interaction may be via mouse and/or keyboard.
- `Container`: Can hold a list of child containers or components.
- `ContainerComponent`: Composition trait of `Container` and `Component`.
- `ComponentTheme`: Defines the `ComponentStyle` for each **Structura** `Component` in terms of its current `ComponentState`.

### Structs

- [x] ContainerComponent: `Row`.
- [x] ContainerComponent: `Column`.
- [x] ContainerComponent: `Panel`: Holds a single `Component`.
- [x] ContainerComponent: `BorderLayout` (North, West, Center, East, South).
- [ ] ContainerComponent: `SplitPane`: Vertical or Horizontal. Contains two children.
- [ ] ContainerComponent: `Tabs`.
- [x] Component: `Button`.
- [ ] Component: `ImageButton`: Generalize the Button to display Text or Image?
- [ ] Component: `Image`.
- [ ] Component: `TextArea`.
- [ ] Component: `TextField`: Subset of `TextArea`? Or create a `multi_line` field on `TextArea`.
- [ ] Component: `Label`.
- [ ] Component: `ToolTip`.
- [ ] Component: `List`.
- [ ] Component: `Tree`.
- [ ] Component: `Combobox`.

### Core API

- [ ] Model.
- [ ] View.
- [ ] Controller.
- [x] `Application`.
- [ ] `ComponentTheme`.
- [ ] `ComponentStyle`.
- [x] `ComponentState`.
- [ ] Event or Message system.
- [ ] Consolidate text rendering.
- [ ] Consolidate draw functions, such as `draw_border()`.
- [ ] Resizeable containers that change size with the window size and automatically resize child components.
- [ ] Add "parent" field to Containers. Resizeable containers can listen for parent container resizing.
- [ ] Add font field to TextArea.
- [ ] Modify TextArea to make scrollbars clickable and draggable. (Works but many bugs).
- [ ] Add support for selectable text in TextArea.
- [ ] Add support for line numbers in TextArea.
- [ ] Add support for NEWLINE characters in TextArea.
- [ ] Add support for TAB key to move between controls. Need to track current focused `Component` in the `Application`.
- [ ] Window: Dialog support?
