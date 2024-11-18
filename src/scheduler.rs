use std::future::Future;

#[derive(Debug, Default)]
pub struct Scheduler {
    smol_executor: smol::Executor<'static>,
}

impl Scheduler {
    pub fn run(&self, closed_broadcast: async_broadcast::Receiver<()>) {
        std::thread::scope(|scope| {
            let thread_nums = std::thread::available_parallelism().map_or(1, |x| x.get());

            for i in 0..thread_nums {
                let name = format!("LuauApp-thread-{i}");

                std::thread::Builder::new()
                    .name(name)
                    .spawn_scoped(scope, || {
                        let mut closed_inner = closed_broadcast.clone();

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
