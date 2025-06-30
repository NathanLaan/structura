//!
//!
//!

use structura_lib::app::Application;
use structura_lib::component::button::Button;
use structura_lib::container::Container;
use structura_lib::container::Row;
use winit::dpi::PhysicalPosition;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BOX_SIZE: u32 = 100; // Size of the square box

fn main() {
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
    test_button2.position.x = test_button1.position.x + test_button1.size.width as f64;

    let mut row = Row::new(20.0, 20.0, 1, 60);
    row.push(Box::new(Button::new(0, 0, 200, 60, "A".to_string())));
    row.push(Box::new(Button::new(0, 0, 200, 60, "B".to_string())));
    row.push(Box::new(Button::new(0, 0, 200, 60, "C".to_string())));
    row.push(Box::new(test_button1));
    row.push(Box::new(test_button2));

    //
    // TODO: Working, without controls, but causes the code below to panic().
    //
    // TODO: Setup UI "tree".
    //
    // let mut container = Container::new();
    // container.push(row);
    let mut application = Application::new(Box::new(row));
    //
    // TODO: Setup message-handle functionality
    //
    application.run();
}
