use std::sync::Arc;
pub(crate) use stopper::Stopped;
pub use thread::initialize_threads;

mod stopper;
mod thread;

pub const ALWAYS_SINGLE_THREAD: bool = false;

#[macro_export]
macro_rules! main {
    (|$executor:ident, $proxy:ident| $main:block) => {
        use std::sync::Arc;

        fn main() {
            let sched = Scheduler::new();
            let $executor = Arc::clone(&sched.executor);

            scheduler::initialize_threads(sched, move |$proxy| $main);
        }
    };
}

#[derive(Debug)]
pub struct Scheduler {
    pub executor: Arc<smol::Executor<'static>>,
    pub(crate) stopped: Stopped,

    pub(crate) send_proxy: async_broadcast::Sender<crate::app::AppProxy>,
    pub recv_proxy: async_broadcast::Receiver<crate::app::AppProxy>,
}

impl Scheduler {
    pub fn new() -> Self {
        let channel_proxy = async_broadcast::broadcast(1);

        Self {
            executor: Arc::new(smol::Executor::new()),
            stopped: Stopped::new(),

            send_proxy: channel_proxy.0,
            recv_proxy: channel_proxy.1,
        }
    }
}
