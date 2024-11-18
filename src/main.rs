use scheduler::Scheduler;
use std::time::Duration;

// mod app;
pub mod app;
mod scheduler;

fn main() {
    let scheduler = Scheduler::new();

    scheduler
        .executor
        .spawn(async move {
            println!("Hey!");
            smol::Timer::after(Duration::from_secs(1)).await;
            println!("Hi!");
        })
        .detach();

    scheduler::initialize_threads(scheduler, |proxy| {
        println!("{:?}", proxy);
    });
}
