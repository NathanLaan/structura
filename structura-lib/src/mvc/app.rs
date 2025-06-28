//!
//!
//!

use crate::component;
use crate::component::Container;
use crate::geometry::Size;
use crate::view::{BufferContext, ViewContext};
use softbuffer::{Buffer, Context, Surface};
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::rc::Rc;
use tokio::task::{JoinHandle, consume_budget};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

///
/// Structure Application wrapper.
///
pub struct Application {
    pub root: Container,
    pub cursor_pos: Option<(usize, usize)>,
    pub mouse_pressed: bool,
    active_event_loop: Option<ActiveEventLoop>,
    message_join_handle: Option<JoinHandle<()>>,
    //view_context: dyn ViewContext,
    //
    // TODO: Separate UI rendering handle?
    //
    //winit_app:
}

impl Application {
    // TODO: new()
    // TODO: run()
    // TODO: quit()

    ///
    /// Constructor.
    ///
    pub fn new(root: Container) -> Self {
        Self {
            root,
            cursor_pos: None,
            mouse_pressed: false,
            active_event_loop: None,
            message_join_handle: None,
            //view_context: (),
        }
    }

    ///
    /// Initialize and run.
    ///
    pub fn run(&mut self) {
        let title = "Structura.App";
        let width = 1024.0;
        let height = 768.0;

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

    fn handle_events(
        self: &mut Self,
        state: &mut (Rc<Window>, Context<Rc<Window>>),
        surface: Option<&mut Surface<Rc<Window>, Rc<Window>>>,
        event: Event<()>,
        active_event_loop: &ActiveEventLoop,
    ) {
        let window = &state.0;

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
                        width: window_size.width,
                        height: window_size.height,
                    }
                };
                // let (width, height) = {
                //     let size = window.inner_size();
                //     (size.width, size.height)
                // };
                surface
                    .resize(
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    )
                    .unwrap();

                let mut buffer: Buffer<Rc<Window>, Rc<Window>> = surface.buffer_mut().unwrap();
                for y in 0..size.height {
                    // Vertical blue fade from 0 to 255
                    //let blue = (255 * y / height) as u8;
                    //let color = (blue as u32) & 0x0000FF;

                    for x in 0..size.width {
                        // Diagonal fade from black to white
                        let factor =
                            ((x + y) as f32 / (size.width + size.height) as f32).clamp(0.0, 1.0);
                        let gray = (factor * 255.0) as u8;
                        let color = ((gray as u32) << 16) | ((gray as u32) << 8) | gray as u32;
                        let idx = y * size.width + x;
                        buffer[idx as usize] = color;
                    }
                }

                // Fill background
                // for pixel in buffer.iter_mut() {
                //     *pixel = 0x111111;
                // }

                let mut buffer_context = BufferContext {
                    buffer: buffer,
                    screen_size: size,
                    font: &component::load_font(),
                    font_size: 32.0,
                };

                for comp in &self.root.children {
                    comp.draw(&mut buffer_context);
                }

                //
                // Draw the button
                //
                // test_button1.draw(
                //     &mut buffer,
                //     size.width as usize,
                //     size.clone(),
                //     &component::load_font(),
                //     32.0,
                // );
                // test_button2.draw(
                //     &mut buffer,
                //     size.width as usize,
                //     size.clone(),
                //     &component::load_font(),
                //     32.0,
                // );

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
                        device_id,
                        position,
                    },
                window_id,
            } if window_id == window.id() => {
                //
                // TODO: Crawl the component tree
                //

                for comp in &self.root.children {
                    //comp.draw(&mut buffer_context);
                }

                // cursor_pos = Some(position);
                // if let Some(pos) = cursor_pos {
                //     //
                //     // TODO: This is where we would crawl the component tree
                //     //
                //     test_button1.update_state(pos.x as usize, pos.y as usize, mouse_pressed);
                //     test_button2.update_state(pos.x as usize, pos.y as usize, mouse_pressed);
                //     window.request_redraw();
                // }
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
                // if let Some(pos) = cursor_pos {
                //     let x = pos.x as usize;
                //     let y = pos.y as usize;
                //     test_button1.handle_mouse_event(x, y, state == ElementState::Pressed);
                //     test_button2.handle_mouse_event(x, y, state == ElementState::Pressed);
                //     // Handle click
                //     // if test_button.was_clicked {
                //     //     println!("Button clicked!");
                //     // }
                // }

                // match state {
                //     ElementState::Pressed => {
                //         mouse_pressed = true;
                //         if let Some(pos) = cursor_pos {
                //             let x = pos.x as usize;
                //             let y = pos.y as usize;
                //             test_button.component_state = ComponentState::Pressed;
                //             if test_button.contains(x, y) {
                //                 println!("test_button pressed!");
                //             }
                //         }
                //     }
                //     ElementState::Released => {
                //         if let Some(pos) = cursor_pos {
                //             test_button.component_state = ComponentState::Active;
                //             let x = pos.x as usize;
                //             let y = pos.y as usize;
                //             if test_button.contains(x, y) {
                //                 println!("test_button released!");
                //                 test_button.component_state = ComponentState::Hovered;
                //             }
                //         }
                //         mouse_pressed = false;
                //     }
                // }
                window.request_redraw();
            }

            _ => {}
        }
    }
}

///
/// Internal definition for constructable winit application.
///
pub struct WinitApp<ContainedState, SurfaceState, Init, InitSurface, Handler> {
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

/// Run a Winit application.
#[allow(unused_mut)]
pub fn _run_app(event_loop: EventLoop<()>, mut app: impl ApplicationHandler<()> + 'static) {
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    event_loop.run_app(&mut app).unwrap();

    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    winit::platform::web::EventLoopExtWebSys::spawn_app(event_loop, app);
}

/// Create a window from a set of window attributes.
#[allow(dead_code)]
pub fn _make_window(
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
pub struct WinitAppBuilder<ContainedState, SurfaceState, Init, InitSurface> {
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
