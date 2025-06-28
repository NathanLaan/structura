//!
//!
//!

use structura_lib::app::{Application, WinitApp};
use structura_lib::component::button::Button;
use structura_lib::component::{ComponentState, Container};
use winit::dpi::PhysicalPosition;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BOX_SIZE: u32 = 100; // Size of the square box

fn main() {
    let mut cursor_pos: Option<PhysicalPosition<f64>> = None;
    let mut mouse_pressed = false;

    let mut test_button1 = Button::default().on_click(|| {
        println!("test_button1.on_click()");
    });
    test_button1.set_text("Button 1!".to_string());
    let mut test_button2 = Button::default().on_click(|| {
        println!("test_button2.on_click()");
    });
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
