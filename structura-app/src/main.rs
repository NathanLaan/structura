//!
//!
//!

use softbuffer::{Context, Surface};
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::rc::Rc;
use structura_lib::component;
use structura_lib::component::button::Button;
use structura_lib::geometry::Size;
use structura_lib::view::View;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BOX_SIZE: u32 = 100; // Size of the square box

fn main() {
    let mut cursor_pos: Option<PhysicalPosition<f64>> = None;

    let test_button = Button {
        x: 100,
        y: 100,
        width: 150,
        height: 50,
        color: 0x0077CC, // blue
        label: "Button!",
        on_clicked: None,
    };

    let mut app = WinitAppBuilder::with_init(
        |elwt| {
            let window = Rc::new(
                elwt.create_window(
                    Window::default_attributes()
                        .with_title("Structura.App")
                        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
                )
                .unwrap(),
            );
            let context = Context::new(window.clone()).unwrap();
            (window, context)
        },
        |_elwt, (window, context)| Surface::new(context, window.clone()).unwrap(),
    )
    .with_event_handler(|(window, _context), surface, event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let size = {
                    let window_size = window.inner_size();
                    Size {
                        width: window_size.width as f32,
                        height: window_size.height as f32,
                    }
                };
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                for y in 0..height {
                    // Vertical blue fade from 0 to 255
                    //let blue = (255 * y / height) as u8;
                    //let color = (blue as u32) & 0x0000FF;

                    for x in 0..width {
                        // Diagonal fade from black to white
                        let factor = ((x + y) as f32 / (width + height) as f32).clamp(0.0, 1.0);
                        let gray = (factor * 255.0) as u8;
                        let color = ((gray as u32) << 16) | ((gray as u32) << 8) | gray as u32;
                        let idx = y * width + x;
                        buffer[idx as usize] = color;
                    }
                }

                // Fill background
                // for pixel in buffer.iter_mut() {
                //     *pixel = 0x111111;
                // }

                // Draw the button
                test_button.draw(&mut buffer, width as usize, size, &component::load_font());

                buffer.present().unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }

            Event::WindowEvent {
                event:
                    WindowEvent::CursorMoved {
                        device_id,
                        position,
                    },
                window_id,
            } if window_id == window.id() => {
                cursor_pos = Some(position);
            }

            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                window_id,
            } if window_id == window.id() => {
                if let Some(pos) = cursor_pos {
                    let x = pos.x as usize;
                    let y = pos.y as usize;
                    if test_button.contains(x, y) {
                        println!("test_button clicked!");
                    }
                }
            }

            _ => {}
        }
    });

    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}

///
/// Draws a box in the center of the frame.
/// `frame`: Mutable slice of bytes representing the pixel buffer (10-byte RGBA format).
/// `frame_width`: Width of the frame buffer.
/// `frame_height`: Height of the frame buffer.
///
fn draw_box(frame: &mut [u8], frame_width: u32, frame_height: u32) {
    // Center the box
    let start_x = (frame_width - BOX_SIZE) / 2;
    let start_y = (frame_height - BOX_SIZE) / 2;

    // Color
    let r = 0xFF;
    let g = 0x00;
    let b = 0x00;
    let a = 0xFF;

    for y in 0..BOX_SIZE {
        for x in 0..BOX_SIZE {
            let abs_x = start_x + x;
            let abs_y = start_y + y;

            if abs_x < frame_width && abs_y < frame_height {
                // Calculate index in 1D byte array for the current pixel (RGBA)
                let index = ((abs_y * frame_width + abs_x) * 4) as usize;
                frame[index] = r;
                frame[index + 1] = g;
                frame[index + 2] = b;
                frame[index + 3] = a;
            }
        }
    }
}

/// Run a Winit application.
#[allow(unused_mut)]
pub(crate) fn run_app(event_loop: EventLoop<()>, mut app: impl ApplicationHandler<()> + 'static) {
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    event_loop.run_app(&mut app).unwrap();

    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    winit::platform::web::EventLoopExtWebSys::spawn_app(event_loop, app);
}

/// Create a window from a set of window attributes.
#[allow(dead_code)]
pub(crate) fn make_window(
    elwt: &ActiveEventLoop,
    f: impl FnOnce(WindowAttributes) -> WindowAttributes,
) -> Rc<Window> {
    let attributes = f(WindowAttributes::default());
    #[cfg(target_arch = "wasm32")]
    let attributes = winit::platform::web::WindowAttributesExtWebSys::with_append(attributes, true);
    let window = elwt.create_window(attributes);
    Rc::new(window.unwrap())
}

/// Easily constructable winit application.
pub(crate) struct WinitApp<T, S, Init, InitSurface, Handler> {
    /// Closure to initialize `state`.
    init: Init,

    /// Closure to initialize `surface_state`.
    init_surface: InitSurface,

    /// Closure to run on window events.
    event: Handler,

    /// Contained state.
    state: Option<T>,

    /// Contained surface state.
    surface_state: Option<S>,
}

/// Builder that makes it so we don't have to name `T`.
pub(crate) struct WinitAppBuilder<T, S, Init, InitSurface> {
    /// Closure to initialize `state`.
    init: Init,

    /// Closure to initialize `surface_state`.
    init_surface: InitSurface,

    /// Eat the type parameter.
    _marker: PhantomData<(Option<T>, Option<S>)>,
}

impl<T, S, Init, InitSurface> WinitAppBuilder<T, S, Init, InitSurface>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
{
    /// Create with an "init" closure.
    pub(crate) fn with_init(init: Init, init_surface: InitSurface) -> Self {
        Self {
            init,
            init_surface,
            _marker: PhantomData,
        }
    }

    /// Build a new application.
    pub(crate) fn with_event_handler<F>(self, handler: F) -> WinitApp<T, S, Init, InitSurface, F>
    where
        F: FnMut(&mut T, Option<&mut S>, Event<()>, &ActiveEventLoop),
    {
        WinitApp::new(self.init, self.init_surface, handler)
    }
}

impl<T, S, Init, InitSurface, Handler> WinitApp<T, S, Init, InitSurface, Handler>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
    Handler: FnMut(&mut T, Option<&mut S>, Event<()>, &ActiveEventLoop),
{
    /// Create a new application.
    pub(crate) fn new(init: Init, init_surface: InitSurface, event: Handler) -> Self {
        Self {
            init,
            init_surface,
            event,
            state: None,
            surface_state: None,
        }
    }
}

impl<T, S, Init, InitSurface, Handler> ApplicationHandler
    for WinitApp<T, S, Init, InitSurface, Handler>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
    Handler: FnMut(&mut T, Option<&mut S>, Event<()>, &ActiveEventLoop),
{
    fn resumed(&mut self, el: &ActiveEventLoop) {
        debug_assert!(self.state.is_none());
        let mut state = (self.init)(el);
        self.surface_state = Some((self.init_surface)(el, &mut state));
        self.state = Some(state);
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        let surface_state = self.surface_state.take();
        debug_assert!(surface_state.is_some());
        drop(surface_state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        let surface_state = self.surface_state.as_mut();
        (self.event)(
            state,
            surface_state,
            Event::WindowEvent { window_id, event },
            event_loop,
        );
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(state) = self.state.as_mut() {
            (self.event)(
                state,
                self.surface_state.as_mut(),
                Event::AboutToWait,
                event_loop,
            );
        }
    }
}
