use scheduler::Scheduler;

pub mod app;
mod scheduler;

pub const ALWAYS_SINGLE_THREAD: bool = true;
pub const WINDOW_DEFAULT_TITLE: &str = "LuauApp";

main!(|sched, proxy| {
    let window = proxy.create_window(Some("Application".to_owned()));
    println!("{window:?}");
});
