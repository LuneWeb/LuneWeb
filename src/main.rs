use scheduler::Scheduler;

// mod app;
mod scheduler;

fn manage_scheduler(scheduler: Scheduler) {
    let threads_count = std::thread::available_parallelism()
        .map_or(1, |x| x.get())
        .clamp(1, 8);

    if threads_count == 1 {
        // single thread
        std::thread::Builder::new()
            .name("user-thread".to_owned())
            .spawn(move || smol::block_on(scheduler.executor.run(scheduler.stopped.wait())))
            .expect("Failed to create thread");
    } else {
        // multi thread
        std::thread::scope(|scope| {
            for i in 0..threads_count {
                let executor = &scheduler.executor;
                let stopped = &scheduler.stopped;

                std::thread::Builder::new()
                    .name(format!("user-thread-{i}"))
                    .spawn_scoped(scope, || smol::block_on(executor.run(stopped.wait())))
                    .expect("Failed to create thread");
            }
        });
    }
}

fn main() {
    let scheduler = Scheduler::new();

    manage_scheduler(scheduler);
}
