//!
//!
//!

use softbuffer::{Context, Surface};
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::rc::Rc;
use structura_lib::app::WinitAppBuilder;
use structura_lib::app::{Application, WinitApp};
use structura_lib::component;
use structura_lib::component::button::Button;
use structura_lib::component::{ComponentState, Container};
use structura_lib::geometry::Size;
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
    let mut mouse_pressed = false;

    let mut test_button1 = Button::default();
    test_button1.set_text("Button 1!".to_string());
    let mut test_button2 = Button::default();
    test_button2.set_text("Button 2!".to_string());
    test_button2.x = test_button1.x + test_button1.width;

    //
    // TODO: Working, without controls, but causes the code below to panic().
    //
    // TODO: Setup UI "tree".
    //
    let mut container = Container::new();
    container.push(test_button1);
    container.push(test_button2);
    let mut application = Application::new(container);
    //
    // TODO: Setup message-handle functionality
    //
    application.run();
}
