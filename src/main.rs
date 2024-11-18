use scheduler::{Scheduler, Stopped};
use std::time::Duration;

// mod app;
mod scheduler;

pub const ALWAYS_SINGLE_THREAD: bool = false;

fn initialize_tao(stopped: Stopped) {
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    use tao::platform::unix::EventLoopBuilderExtUnix;

    #[cfg(target_os = "windows")]
    use tao::platform::windows::EventLoopBuilderExtWindows;

    let event_loop = tao::event_loop::EventLoopBuilder::new()
        .with_any_thread(true)
        .build();

    event_loop.run(move |event, _target, control_flow| {
        *control_flow = tao::event_loop::ControlFlow::Exit;
        stopped.stop();
    })
}

fn initialize_threads(scheduler: Scheduler) {
    let threads_count = std::thread::available_parallelism()
        .map_or(1, |x| x.get())
        .clamp(1, 8);

    if threads_count == 1 || ALWAYS_SINGLE_THREAD {
        // single thread
        let stopped = scheduler.stopped.clone();

        std::thread::Builder::new()
            .name("user-thread".to_owned())
            .spawn(move || smol::block_on(scheduler.executor.run(scheduler.stopped.wait())))
            .expect("Failed to create thread");

        initialize_tao(stopped);
    } else {
        // multi thread
        std::thread::scope(|scope| {
            std::thread::Builder::new()
                .name(format!("tao-thread"))
                .spawn_scoped(scope, || initialize_tao(scheduler.stopped.clone()))
                .expect("Failed to create thread");

            for i in 1..threads_count {
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

    if ALWAYS_SINGLE_THREAD {
        println!("[warn] ALWAYS_SINGLE_THREAD is set to true");
    }

    scheduler
        .executor
        .spawn(async move {
            println!("Hey!");
            smol::Timer::after(Duration::from_secs(1)).await;
            println!("Hi!");
        })
        .detach();

    initialize_threads(scheduler);
}
