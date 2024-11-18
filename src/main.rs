use scheduler::Scheduler;
use std::time::Duration;

// mod app;
mod scheduler;

pub const ALWAYS_SINGLE_THREAD: bool = false;

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

    scheduler::initialize_threads(scheduler);
}
