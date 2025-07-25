//!
//! Structura
//!
use crate::message::Message;

pub trait Controller {
    fn handle_message(&mut self, message: dyn Message);
}
