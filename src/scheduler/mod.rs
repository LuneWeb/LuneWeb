pub(crate) use stopper::Stopped;
pub use thread::initialize_threads;

mod stopper;
mod thread;

#[derive(Debug)]
pub struct Scheduler {
    pub executor: smol::Executor<'static>,
    pub(crate) stopped: Stopped,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            executor: smol::Executor::new(),
            stopped: Stopped::new(),
        }
    }
}
