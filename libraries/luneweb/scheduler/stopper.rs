use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Clone)]
pub(crate) struct Stopped {
    event: Arc<async_event::Event>,
    flag: Arc<AtomicBool>,
}

impl Debug for Stopped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.flag.fmt(f)
    }
}

impl Stopped {
    pub(super) fn new() -> Self {
        Self {
            event: Arc::new(async_event::Event::new()),
            flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub(crate) async fn wait(&self) {
        if self.flag.load(Ordering::Relaxed) {
            return;
        }

        self.event
            .wait_until(|| self.flag.load(Ordering::Relaxed).then_some(()))
            .await;
    }

    pub(crate) fn stop(&self) {
        self.flag.store(true, Ordering::Relaxed);
        self.event.notify_all();
    }
}
