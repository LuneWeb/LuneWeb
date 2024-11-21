use std::sync::Arc;
pub(crate) use stopper::Stopped;

mod stopper;
pub mod thread;

#[derive(Debug, Clone)]
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
