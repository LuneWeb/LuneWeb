use scheduler::Scheduler;

// mod app;
pub mod app;
mod scheduler;

main!(|executor, proxy| {
    let window = proxy.create_window();

    executor
        .spawn(async move {
            println!("spawned");
        })
        .detach();

    println!("{:?}", window);
});
