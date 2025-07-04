//!
//! Structura Test Application.
//!

use std::cell::RefCell;
use std::rc::Rc;
use structura_lib::app::Application;
use structura_lib::component::ComponentHandle;
use structura_lib::component::button::Button;
use structura_lib::component::textarea::TextArea;
use structura_lib::container::Container;
use structura_lib::container::column::Column;
use structura_lib::container::row::Row;
use structura_lib::geometry::{Point, Size};

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
        textarea1_clone.borrow_mut().text.push_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
    });

    let mut row1 = Row::new(20.0, 20.0, 1, 60);
    row1.push(Box::new(Button::new(0, 0, 200, 60, "A".to_string())));
    row1.push(Box::new(Button::new(0, 0, 200, 60, "B".to_string())));
    row1.push(Box::new(Button::new(0, 0, 200, 60, "C".to_string())));
    row1.push(Box::new(test_button1));
    row1.push(Box::new(test_button2));
    row1.push(Box::new(button_set_text));

    let mut row2 = Row::new(20.0, 20.0, 1, 200);
    row2.push(Box::new(ComponentHandle::new(textarea1)));

    let mut col = Column::new(0.0, 0.0, 10, 1000, 1000);
    col.push(Box::new(row1));
    col.push(Box::new(row2));

    let mut application = Application::new(Box::new(col));
    application.run();
}
