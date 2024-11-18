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
        let (sender, receiver) = flume::bounded(1);
        proxy
            .send_event(app::AppEvent::CreateWindow(sender))
            .expect("Failed to send event");
        let window = receiver.recv().expect("Failed to receive window");

        println!("{:?}", window);
    });
}
