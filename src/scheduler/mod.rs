pub(crate) use stopper::Stopped;
pub use thread::initialize_threads;

mod stopper;
mod thread;

pub const ALWAYS_SINGLE_THREAD: bool = false;

#[derive(Debug)]
pub struct Scheduler {
    pub executor: smol::Executor<'static>,
    pub(crate) stopped: Stopped,

    pub(crate) send_proxy:
        async_broadcast::Sender<tao::event_loop::EventLoopProxy<crate::app::AppEvent>>,
    pub recv_proxy:
        async_broadcast::Receiver<tao::event_loop::EventLoopProxy<crate::app::AppEvent>>,
}

impl Scheduler {
    pub fn new() -> Self {
        let channel_proxy = async_broadcast::broadcast(1);

        Self {
            executor: smol::Executor::new(),
            stopped: Stopped::new(),

            send_proxy: channel_proxy.0,
            recv_proxy: channel_proxy.1,
        }
    }
}
