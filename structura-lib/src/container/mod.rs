//!
//! Structura
//!
//! Container types
//!

pub mod column;
pub mod row;

use crate::component::Component;

/// Container node for building widget trees
pub trait Container {
    fn push(&mut self, component: Box<dyn Component>);

    ///
    /// Force the `Container` to update the position of controls within the `Container`.
    ///
    fn update_layout(&mut self);
}

///
/// Combination `Container` + `Component` trait.
///
pub trait ContainerComponent: Container + Component {}
impl<T> ContainerComponent for T where T: Container + Component {}
