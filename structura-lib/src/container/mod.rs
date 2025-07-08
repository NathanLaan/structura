//!
//! Structura
//!
//! Container types
//!

pub mod border;
pub mod column;
pub mod panel;
pub mod row;

use crate::component::Component;

///
/// Container node for building UI component trees.
///
pub trait Container {
    ///
    /// Add the specified `Component` as a child to the `Container`.
    ///
    fn push(&mut self, component: Box<dyn Component>);

    ///
    /// Called when the parent `Container` resizes.
    ///
    /// Passes in the parent container `width` and `height`.
    ///
    fn resize(&mut self, width: usize, height: usize);

    ///
    /// Force the `Container` to update the position of controls within the `Container`.
    ///
    fn update_layout(&mut self);

    ///
    /// Determines whether the `Container` fills the parent container it is added to.
    ///
    /// When a container returns true from `fills_parent_container()`:
    ///
    /// 1. When that container is added to another container, it is resized to fill the parent.
    /// 2. When the parent container resizes, the child is resized to fill.
    ///
    fn fills_parent_container(&self) -> bool;
}

///
/// Composite `Container` + `Component` trait.
///
pub trait ContainerComponent: Container + Component {}
impl<T> ContainerComponent for T where T: Container + Component {}
