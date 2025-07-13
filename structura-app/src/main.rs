//!
//! Structura Test Application.
//!

use std::cell::RefCell;
use std::rc::Rc;
use structura::app::Application;
use structura::component::ComponentHandle;
use structura::component::button::Button;
use structura::component::textarea::TextArea;
use structura::container::Container;
use structura::container::border::BorderLayout;
use structura::container::panel::Panel;
use structura::container::row::Row;
use structura::geometry::{Point, Size};

fn main() {
    let textarea1 = Rc::new(RefCell::new(TextArea::new()));
    textarea1.borrow_mut().position = Point { x: 80.0, y: 80.0 };
    textarea1.borrow_mut().size = Size {
        width: 700,
        height: 300,
    };
    let textarea1_clone = textarea1.clone();

    let test_button1 = Button::default()
        .set_text("Button 1!".to_string())
        .on_click(|| {
            println!("test_button1.on_click()");
        });
    let test_button2 = Button::default()
        .set_text("Button 2!".to_string())
        .on_click(|| {
            println!("test_button2.on_click()");
        });
    let button_set_text = Button::default().set_text("Add Text".to_string()).on_click(move|| {
        textarea1_clone.borrow_mut().insert_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
    });

    let mut row1 = Row::new(20.0, 20.0, 1, 60);
    row1.push(Box::new(Button::new(0, 0, 200, 60, "A".to_string())));
    row1.push(Box::new(Button::new(0, 0, 200, 60, "B".to_string())));
    row1.push(Box::new(Button::new(0, 0, 200, 60, "C".to_string())));
    row1.push(Box::new(test_button1));
    row1.push(Box::new(test_button2));
    row1.push(Box::new(button_set_text));

    let mut panel = Panel::new();
    panel.push(Box::new(ComponentHandle::new(textarea1)));

    let mut main_container = BorderLayout::new();
    main_container.set_north(Box::new(row1));
    main_container.set_center(Box::new(panel));

    let mut application = Application::new(Box::new(main_container));
    application.run();
}
