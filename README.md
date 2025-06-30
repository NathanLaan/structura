# Structura Rust GUI Framework

Structura is a basic GUI framework created to learn how to create a GUI framework in Rust. Maybe one day it will be "OK enough" to actually use for something, but you really should not use this.

## Dependencies

Structura is based on the following:

- [winit](https://docs.rs/winit/latest/winit/) cross-platform window management library.
- [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/) 2D buffer library.
- [rusttype](https://docs.rs/rusttype/latest/rusttype/) for fonts.
- [Tokio](https://docs.rs/tokio/latest/tokio/) for asynchronous functions.

## Architecture

Structura is being loosely designed around a Model-View-Controller (MVC) architecture.

### Traits

- Component: Displays output to users and/or allows users to interact. Interaction may be via mouse and/or keyboard.
- Container: Can hold a list of child items.

### Structs

- Row: Container
- Column: Container
- BorderLayout: Container
- Button: Component
- TextField: Component

## TODO

- [ ] Buttons.
- [x] Button on_click event handler.
- [ ] Text fields.
- [ ] Layouts.
- [ ] Theme.
- [ ] Style support.
- [ ] Model.
- [ ] View.
- [ ] Controller.
- [ ] Application.
- [ ] Event or Message system.
- [ ] Trait-based API for creating Structura-based apps.
