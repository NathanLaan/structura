//!
//! Structura: Application wrapper/helper struct.
//!

use crate::component;
use crate::component::style::{ComponentTheme, DefaultComponentTheme};
use crate::container::ContainerComponent;
use crate::geometry::{Point, Size};
use crate::view::BufferContext;
use softbuffer::{Buffer, Context, Surface};
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowAttributes, WindowId};
//
// TODO: Track the component that currently has focus?
//

///
/// Structure Application wrapper.
///
pub struct Application {
    pub root: Box<dyn ContainerComponent>,
    pub cursor_pos: Option<Point>,
    pub mouse_pressed: bool,
    pub theme: Box<dyn ComponentTheme>,
    //
    // TODO: Separate UI rendering handle?
    //
    //active_event_loop: Option<ActiveEventLoop>,
    //message_join_handle: Option<JoinHandle<()>>,
    //winit_app:
}

impl Application {
    // TODO: new()
    // TODO: run()
    // TODO: quit()

    ///
    /// Constructor.
    ///
    pub fn new(root: Box<dyn ContainerComponent>) -> Self {
        Self {
            root,
            cursor_pos: None,
            mouse_pressed: false,
            theme: Box::new(DefaultComponentTheme::default()),
            //message_join_handle: None,
        }
    }

    ///
    /// Initialize and run.
    ///
    pub fn run(&mut self) {
        let title = "Structura.App";
        let width = 1280.0;
        let height = 1024.0;

        let mut app = WinitAppBuilder::create_winit_app(
            |elwt| Application::create_window_and_context(elwt, title, width, height),
            Application::create_surface,
        )
        //.with_event_handler(Application::handle_events);
        .with_event_handler({
            let this = self; // capture mutable self
            move |state, surface, event, elwt| this.handle_events(state, surface, event, elwt)
        });

        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut app).unwrap();
    }

    ///
    /// Used by `Application::run()` in the closure that calls `WinitAppBuilder::create_winit_app(...)`.
    ///
    fn create_window_and_context(
        elwt: &ActiveEventLoop,
        title: &str, // TODO: Move title+w+h to a new 'WindowSettings' struct
        w: f64,
        h: f64,
    ) -> (Rc<Window>, Context<Rc<Window>>) {
        let window = Rc::new(
            elwt.create_window(
                Window::default_attributes()
                    .with_title(title)
                    .with_inner_size(LogicalSize::new(w, h)),
            )
            .unwrap(),
        );
        let context = Context::new(window.clone()).unwrap();
        (window, context)
    }

    ///
    /// Used by `Application::run()` in the closure that calls `WinitAppBuilder::create_winit_app(...)`.
    ///
    fn create_surface(
        _elwt: &ActiveEventLoop,
        state: &mut (Rc<Window>, Context<Rc<Window>>),
    ) -> Surface<Rc<Window>, Rc<Window>> {
        let (window, context) = state;
        Surface::new(context, window.clone()).unwrap()
    }

    ///
    /// Handle application events.
    ///
    fn handle_events(
        self: &mut Self,
        state: &mut (Rc<Window>, Context<Rc<Window>>),
        surface: Option<&mut Surface<Rc<Window>, Rc<Window>>>,
        event: Event<()>,
        active_event_loop: &ActiveEventLoop,
    ) {
        let window = &state.0;
        let scale_factor = window.scale_factor();
        //
        // The following code should work but does not seem to work on Linux Wayland.
        //
        // let mut scale_factor = 1.0;
        // if let Some(h) = window.current_monitor() {
        //     scale_factor = h.scale_factor();
        // }

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("Error: RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let size = {
                    let window_size = window.inner_size();
                    Size {
                        width: window_size.width,
                        height: window_size.height,
                    }
                };
                surface
                    .resize(
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    )
                    .unwrap();

                let mut buffer: Buffer<Rc<Window>, Rc<Window>> = surface.buffer_mut().unwrap();
                for y in 0..size.height {
                    //
                    // Vertical blue fade from 0 to 255
                    //
                    //let blue = (255 * y / height) as u8;
                    //let color = (blue as u32) & 0x0000FF;
                    //
                    // Diagonal fade from black to white
                    //
                    for x in 0..size.width {
                        let factor =
                            ((x + y) as f32 / (size.width + size.height) as f32).clamp(0.0, 1.0);
                        let gray = (factor * 255.0) as u8;
                        let color = ((gray as u32) << 16) | ((gray as u32) << 8) | gray as u32;
                        let idx = y * size.width + x;
                        buffer[idx as usize] = color;
                    }
                }
                //
                // (1) Prepare the BufferContext
                // (2) Draw all Components/ContainerComponents
                // (3) Present/Render the buffer to the screen
                //
                let mut buffer_context = BufferContext {
                    buffer: buffer,
                    screen_size: size,
                    font: &component::load_font(),
                    font_size: 32.0,
                    theme: &self.theme,
                };
                self.root.draw(&mut buffer_context);
                buffer_context.buffer.present().unwrap();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                active_event_loop.exit();
            }

            Event::WindowEvent {
                event:
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                    },
                window_id,
            } if window_id == window.id() => {
                let mouse_input = crate::event::MouseInput {
                    scale_factor,
                    position: Point {
                        x: position.x,
                        y: position.y,
                    },
                    pressed: false,
                    just_released: false,
                    mouse_scroll: None,
                };
                self.cursor_pos = Some(mouse_input.position);
                self.root.handle_mouse_event(mouse_input);
                window.request_redraw();
            }

            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event,
                        is_synthetic: _,
                    },
                window_id,
            } if window_id == window.id() => {
                //println!("{:?} {:?}", event, window_id);
                self.root.handle_keyboard_event(&event);
                window.request_redraw();
            }

            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state,
                        button: MouseButton::Left,
                        ..
                    },
                window_id,
            } if window_id == window.id() => {
                if let Some(pos) = self.cursor_pos {
                    let mut mouse_input = crate::event::MouseInput {
                        scale_factor,
                        position: Point { x: pos.x, y: pos.y },
                        pressed: true,
                        just_released: false,
                        mouse_scroll: None,
                    };

                    // TODO: This could be a lot more compact... `mouse_input.pressed = ...`
                    match state {
                        ElementState::Pressed => {
                            mouse_input.pressed = true;
                        }
                        ElementState::Released => {
                            mouse_input.pressed = false;
                            mouse_input.just_released = true;
                        }
                    }
                    self.root.handle_mouse_event(mouse_input);
                }
                window.request_redraw();
            }

            Event::WindowEvent {
                event:
                    WindowEvent::MouseWheel {
                        device_id: _,
                        delta,
                        phase,
                    },
                window_id,
            } if window_id == window.id() => {
                self.root.handle_mouse_wheel_event(&delta, &phase);
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                window_id,
            } if window_id == window.id() => {
                if self.root.fills_parent_container() {
                    self.root
                        .set_size(size.width as usize, size.height as usize);
                }
                self.root.resize(size.width as usize, size.height as usize);
                window.request_redraw();
            }

            _ => {}
        }
    }
}

///
/// Internal definition for constructable winit application.
///
struct WinitApp<ContainedState, SurfaceState, Init, InitSurface, Handler> {
    /// Closure to initialize `state`.
    init: Init,

    /// Closure to initialize `surface_state`.
    init_surface: InitSurface,

    /// Closure to run on window events.
    event: Handler,

    /// Contained state.
    state: Option<ContainedState>,

    /// Contained surface state.
    surface_state: Option<SurfaceState>,
}

impl<ContainedState, SurfaceState, Init, InitSurface, Handler> ApplicationHandler
    for WinitApp<ContainedState, SurfaceState, Init, InitSurface, Handler>
where
    Init: FnMut(&ActiveEventLoop) -> ContainedState,
    InitSurface: FnMut(&ActiveEventLoop, &mut ContainedState) -> SurfaceState,
    Handler: FnMut(&mut ContainedState, Option<&mut SurfaceState>, Event<()>, &ActiveEventLoop),
{
    fn resumed(&mut self, el: &ActiveEventLoop) {
        debug_assert!(self.state.is_none());
        let mut state = (self.init)(el);
        self.surface_state = Some((self.init_surface)(el, &mut state));
        self.state = Some(state);
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

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        let surface_state = self.surface_state.take();
        debug_assert!(surface_state.is_some());
        drop(surface_state);
    }
}

/// Run a Winit application.
#[allow(unused_mut)]
fn _run_app(event_loop: EventLoop<()>, mut app: impl ApplicationHandler<()> + 'static) {
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    event_loop.run_app(&mut app).unwrap();

    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    winit::platform::web::EventLoopExtWebSys::spawn_app(event_loop, app);
}

/// Create a window from a set of window attributes.
#[allow(dead_code)]
fn _make_window(
    elwt: &ActiveEventLoop,
    f: impl FnOnce(WindowAttributes) -> WindowAttributes,
) -> Rc<Window> {
    let attributes = f(WindowAttributes::default());
    #[cfg(target_arch = "wasm32")]
    let attributes = winit::platform::web::WindowAttributesExtWebSys::with_append(attributes, true);
    let window = elwt.create_window(attributes);
    Rc::new(window.unwrap())
}

///
/// Utility to encapsulate the process to stand up a WinitApp.
///
struct WinitAppBuilder<ContainedState, SurfaceState, Init, InitSurface> {
    /// Closure to initialize `state`.
    init: Init,
    /// Closure to initialize `surface_state`.
    init_surface: InitSurface,
    /// Eat the type parameter.
    _marker: PhantomData<(Option<ContainedState>, Option<SurfaceState>)>,
}

impl<ContainedState, SurfaceState, Init, InitSurface>
    WinitAppBuilder<ContainedState, SurfaceState, Init, InitSurface>
where
    Init: FnMut(&ActiveEventLoop) -> ContainedState,
    InitSurface: FnMut(&ActiveEventLoop, &mut ContainedState) -> SurfaceState,
{
    /// Create with an "init" closure.
    pub fn create_winit_app(init: Init, init_surface: InitSurface) -> Self {
        Self {
            init,
            init_surface,
            _marker: PhantomData,
        }
    }

    /// Build a new application.
    pub fn with_event_handler<EventHandlerFunc>(
        self,
        handler: EventHandlerFunc,
    ) -> WinitApp<ContainedState, SurfaceState, Init, InitSurface, EventHandlerFunc>
    where
        EventHandlerFunc:
            FnMut(&mut ContainedState, Option<&mut SurfaceState>, Event<()>, &ActiveEventLoop),
    {
        WinitApp::new(self.init, self.init_surface, handler)
    }
}

impl<ContainedState, SurfaceState, Init, InitSurface, Handler>
    WinitApp<ContainedState, SurfaceState, Init, InitSurface, Handler>
where
    Init: FnMut(&ActiveEventLoop) -> ContainedState,
    InitSurface: FnMut(&ActiveEventLoop, &mut ContainedState) -> SurfaceState,
    Handler: FnMut(&mut ContainedState, Option<&mut SurfaceState>, Event<()>, &ActiveEventLoop),
{
    /// Create a new application.
    pub fn new(init: Init, init_surface: InitSurface, event: Handler) -> Self {
        Self {
            init,
            init_surface,
            event,
            state: None,
            surface_state: None,
        }
    }
}
