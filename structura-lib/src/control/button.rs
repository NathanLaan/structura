//!
//! Rust MVC-UI
//!
use crate::event::Callback;

pub struct Button {
    
    pub on_clicked: Callback<()>,
    
}