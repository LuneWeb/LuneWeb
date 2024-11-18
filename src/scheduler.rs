use std::future::Future;

#[derive(Debug)]
pub struct Scheduler {
    smol_executor: smol::Executor<'static>,
    closed_recv: async_broadcast::Receiver<()>,
}

impl Scheduler {
    pub fn new(closed_recv: async_broadcast::Receiver<()>) -> Self {
        Self {
            smol_executor: Default::default(),
            closed_recv,
        }
    }

    pub fn run(&self) {
        std::thread::scope(|scope| {
            let thread_nums = std::thread::available_parallelism().map_or(1, |x| x.get());

            for i in 0..thread_nums {
                let name = format!("LuauApp-thread-{i}");

                std::thread::Builder::new()
                    .name(name)
                    .spawn_scoped(scope, || {
                        let mut closed_inner = self.closed_recv.clone();

                        smol::block_on(self.smol_executor.run(closed_inner.recv()))
                            .expect("Failed to run executor");
                    })
                    .expect("Failed to spawn thread");
            }
        });
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'static) -> smol::Task<T>
    where
        T: Send + 'static,
    {
        self.smol_executor.spawn(future)
    }
}
