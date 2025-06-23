//!
//!
//!

use crate::view::ViewContext;
use tokio::task::JoinHandle;

pub struct ApplicationContext {
    pub view_context: dyn ViewContext,
}

pub struct Application {
    message_join_handle: JoinHandle<()>,
    //
    // TODO: Separate UI rendering handle?
    //
}

impl Application {
    // TODO: new()
    // TODO: run()
    // TODO: quit()
}
