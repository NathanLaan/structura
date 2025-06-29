//!
//!
//!

use structura_lib::app::{Application, WinitApp};
use structura_lib::component::button::Button;
use structura_lib::component::layout::Row;
use structura_lib::component::{ComponentState, Container};
use winit::dpi::PhysicalPosition;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BOX_SIZE: u32 = 100; // Size of the square box

fn main() {
    let mut row = Row::new(20.0, 20.0, 10, 40);

    row.push(Button::new(0, 0, 100, 40, "A".to_string()));
    row.push(Button::new(0, 0, 100, 40, "B".to_string()));
    row.push(Button::new(0, 0, 100, 40, "C".to_string()));

    let mut test_button1 = Button::default().on_click(|| {
        println!("test_button1.on_click()");
    });
    test_button1.set_text("Button 1!".to_string());
    let mut test_button2 = Button::default().on_click(|| {
        println!("test_button2.on_click()");
        // for comp in row.children.iter() {
        //     println!("Comp: {:?}", comp.get_position());
        // }
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
    container.push(row);
    let mut application = Application::new(container);
    //
    // TODO: Setup message-handle functionality
    //
    application.run();
}
