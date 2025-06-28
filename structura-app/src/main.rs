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

    /*
    let mut app = WinitAppBuilder::create_winit_app(
        |active_event_loop: &ActiveEventLoop| {
            let window = Rc::new(
                active_event_loop
                    .create_window(
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
    .with_event_handler(|(window, _context), surface, event, active_event_loop| {
        active_event_loop.set_control_flow(ControlFlow::Wait);

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

                let mut buffer = surface.buffer_mut().unwrap();
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

                //
                // Draw the button
                //
                test_button1.draw(
                    &mut buffer,
                    size.width as usize,
                    size.clone(),
                    &component::load_font(),
                    32.0,
                );
                test_button2.draw(
                    &mut buffer,
                    size.width as usize,
                    size.clone(),
                    &component::load_font(),
                    32.0,
                );

                buffer.present().unwrap();
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
                cursor_pos = Some(position);
                if let Some(pos) = cursor_pos {
                    //
                    // TODO: This is where we would crawl the component tree
                    //
                    test_button1.update_state(pos.x as usize, pos.y as usize, mouse_pressed);
                    test_button2.update_state(pos.x as usize, pos.y as usize, mouse_pressed);
                    window.request_redraw();
                }
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
                if let Some(pos) = cursor_pos {
                    let x = pos.x as usize;
                    let y = pos.y as usize;
                    test_button1.handle_mouse_event(x, y, state == ElementState::Pressed);
                    test_button2.handle_mouse_event(x, y, state == ElementState::Pressed);
                    // Handle click
                    // if test_button.was_clicked {
                    //     println!("Button clicked!");
                    // }
                }

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
    });

    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();

    */
}
