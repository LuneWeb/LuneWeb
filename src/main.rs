mod app;
mod scheduler;

fn main() {
    let app = app::App::default();
    let scheduler = scheduler::Scheduler::default();
    let closed = app.closed.1.clone();

    let (proxy, join) = smol::block_on(app.run());

    scheduler
        .spawn(async move {
            let window = app::proxy::AppProxy::create_window(proxy).await?;
            println!("{window:?}");

            Ok::<_, mlua::Error>(())
        })
        .detach();

    scheduler.run(closed);

    join.join().expect("Failed to join");
}
