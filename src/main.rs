use scheduler::Scheduler;

// mod app;
pub mod app;
mod scheduler;

pub const ALWAYS_SINGLE_THREAD: bool = true;

main!(|sched, proxy| {
    let window = proxy.create_window();
    println!("{window:?}");
});
