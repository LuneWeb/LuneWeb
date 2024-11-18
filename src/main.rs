mod app;
mod scheduler;

fn main() {
    let app = app::App::default();
    let scheduler = scheduler::Scheduler::default();
    let closed = app.closed.1.clone();

    let (proxy, join) = smol::block_on(app.run());

    scheduler
        .spawn(async move {
            let (sender, receiver) = flume::unbounded();

            proxy
                .send_event(app::proxy::AppProxy::CreateWindow {
                    send_window: sender,
                })
                .expect("Failed to send event");
            let window = receiver.recv().unwrap();
            println!("{window:?}");

            Ok::<_, mlua::Error>(())
        })
        .detach();

    scheduler.run(closed);

    join.join().expect("Failed to join");
}
