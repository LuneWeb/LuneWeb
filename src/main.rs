use std::sync::Arc;

mod app;
mod scheduler;

fn main() {
    let app = app::App::default();
    let scheduler = Arc::new(scheduler::Scheduler::default());

    let lua = mlua::Lua::new();
    let chunk = lua.load(std::fs::read_to_string("app.luau").unwrap());

    let closed_broadcast = app.closed.1.clone();
    let (proxy, join) = smol::block_on(app.run());

    scheduler
        .spawn(async move {
            let window = app::proxy::AppProxy::create_window(proxy).await?;
            println!("{window:?}");

            Ok::<_, mlua::Error>(())
        })
        .detach();

    scheduler.spawn(chunk.exec_async()).detach();

    scheduler.run(closed_broadcast);

    join.join().expect("Failed to join");
}
