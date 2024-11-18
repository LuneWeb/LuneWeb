use scheduler::Scheduler;
use std::time::Duration;

// mod app;
pub mod app;
mod scheduler;

main!(|sched, proxy| {
    let window = proxy.create_window();
    println!("{window:?}");
});
