# **Structura** Rust GUI Framework

**Structura** is a GUI framework created to learn how to create a GUI framework in Rust. Maybe one day it will be "OK enough" to actually use for something, but you really should not use this.

The **Structura** GUI framework is based on cross-platform components, but has only been tested on Linux.

## Repository

The **Structura** repository consists of:

- `structura-lib`: The **Structura** GUI library, including all necessary components an dependencies.
- `structura-app`: A basic app to test and demonstrate the `structura-lib` components.

## Dependencies

**Structura** is based on the following libraries:

- [winit](https://docs.rs/winit/latest/winit/) cross-platform window management library.
- [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/) 2D buffer library.
- [rusttype](https://docs.rs/rusttype/latest/rusttype/) font library.
- [Tokio](https://docs.rs/tokio/latest/tokio/), an event-driven, non-blocking I/O platform for writing asynchronous applications.
