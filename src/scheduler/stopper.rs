use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub(crate) struct Stopped {
    event: event_listener::Event,
    flag: AtomicBool,
}

impl Stopped {
    pub(super) fn new() -> Self {
        Self {
            event: event_listener::Event::new(),
            flag: AtomicBool::new(false),
        }
    }

    pub(crate) async fn wait(&self) {
        loop {
            if self.flag.load(Ordering::Relaxed) {
                return;
            }

            event_listener::listener!(&self.event => listener);

            if self.flag.load(Ordering::Acquire) {
                return;
            }

            listener.await;
        }
    }

    pub(crate) fn stop(&self) {
        self.flag.store(true, Ordering::SeqCst);
        self.event.notify_additional(usize::MAX);
    }
}
