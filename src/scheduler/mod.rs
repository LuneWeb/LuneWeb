use std::sync::Arc;
pub(crate) use stopper::Stopped;

mod stopper;
pub mod thread;

#[derive(Debug, Clone)]
pub struct Scheduler {
    pub executor: Arc<smol::Executor<'static>>,
    pub(crate) stopped: Stopped,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            executor: Arc::new(smol::Executor::new()),
            stopped: Stopped::new(),
        }
    }
}
