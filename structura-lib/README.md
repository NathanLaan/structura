# Structura-Lib Rust GUI Framework

**Structura** is a GUI framework created to learn how to create a GUI framework in Rust. Maybe one day it will be "OK enough" to actually use for something, but you really should not use this.

The **Structura** GUI framework is based on cross-platform components, but has only been tested on Linux.

## Dependencies

**Structura** is based on the following libraries:

- [winit](https://docs.rs/winit/latest/winit/) cross-platform window management library.
- [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/) 2D buffer library.
- [rusttype](https://docs.rs/rusttype/latest/rusttype/) font library.
- [Tokio](https://docs.rs/tokio/latest/tokio/), an event-driven, non-blocking I/O platform for writing asynchronous applications.

## Architecture

**Structura** is being loosely designed around a Model-View-Controller (MVC) architecture.

### Traits

- `Component`: Displays output to users and/or allows users to interact. Interaction may be via mouse and/or keyboard.
- `Container`: Can hold a list of child items.
- `ContainerComponent`: Composition trait of `Container` and `Component`.

### Structs

- [x] Row: ContainerComponent
- [ ] Column: ContainerComponent
- [ ] BorderLayout: ContainerComponent (Top, Bottom, Left, Right, Center)
- [ ] Tabs: ContainerComponent
- [x] Button: Component
- [ ] TextArea: Component
- [ ] TextField: Component (subset of `TextArea`?)
- [ ] SplitPane: ContainerComponent. Vertical or Horizontal. Contains two children.

### Core API

- [ ] Model.
- [ ] View.
- [ ] Controller.
- [ ] Theme.
- [ ] Style support.
- [ ] Application.
- [ ] Event or Message system.
- [ ] Consolidate text rendering.
- [ ] Consolidate draw functions, such as `draw_border()`.
