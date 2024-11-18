use std::sync::Arc;
pub(crate) use stopper::Stopped;
pub use thread::initialize_threads;

mod stopper;
mod thread;

pub const ALWAYS_SINGLE_THREAD: bool = true;

#[macro_export]
macro_rules! main {
    (|$sched:ident, $proxy:ident| $main:block) => {
        fn main() {
            let $sched = Scheduler::new();

            scheduler::initialize_threads($sched.clone(), move |$proxy| $main);
        }
    };
}

#[derive(Debug, Clone)]
pub struct Scheduler {
    pub executor: Arc<smol::Executor<'static>>,
    pub lua: mlua::Lua,
    pub(crate) stopped: Stopped,

    pub(crate) send_proxy: async_broadcast::Sender<crate::app::AppProxy>,
    pub recv_proxy: async_broadcast::Receiver<crate::app::AppProxy>,
}

impl Scheduler {
    pub fn new() -> Self {
        let channel_proxy = async_broadcast::broadcast(1);

        Self {
            executor: Arc::new(smol::Executor::new()),
            lua: mlua::Lua::new(),
            stopped: Stopped::new(),

            send_proxy: channel_proxy.0,
            recv_proxy: channel_proxy.1,
        }
    }
}
