//!
//!
//!

use crate::component::Container;
use crate::view::ViewContext;
use softbuffer::Surface;
use std::marker::PhantomData;
use std::rc::Rc;
use tokio::task::{JoinHandle, consume_budget};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

pub struct ApplicationContext {
    pub view_context: dyn ViewContext,
}

pub struct Application {
    message_join_handle: JoinHandle<()>,
    //
    // TODO: Separate UI rendering handle?
    //
}

impl Application {
    // TODO: new()
    // TODO: run()
    // TODO: quit()
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
pub fn run_app(event_loop: EventLoop<()>, mut app: impl ApplicationHandler<()> + 'static) {
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    event_loop.run_app(&mut app).unwrap();

    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    winit::platform::web::EventLoopExtWebSys::spawn_app(event_loop, app);
}

/// Create a window from a set of window attributes.
#[allow(dead_code)]
pub fn make_window(
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

pub struct App {
    pub root: Container,
    pub cursor_pos: Option<(usize, usize)>,
    pub mouse_pressed: bool,
    active_event_loop: Option<ActiveEventLoop>,
    //winit_app:
}

impl App {
    ///
    /// Constructor.
    ///
    pub fn new(root: Container) -> Self {
        Self {
            root,
            cursor_pos: None,
            mouse_pressed: false,
            active_event_loop: None,
        }
    }

    ///
    /// Initialize and run.
    ///
    pub fn run(&mut self) {
        //let event_loop = EventLoop::new().unwrap();
        //event_loop.run_app(&mut app).unwrap();
    }
}
