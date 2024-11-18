mod app;

fn main() {
    let executor = smol::Executor::new();

    let app = app::App::default();
    let closed = app.closed.1.clone();

    let (proxy, join) = smol::block_on(app.run());

    executor
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

    std::thread::scope(|scope| {
        let thread_nums = std::thread::available_parallelism().map_or(1, |x| x.get());

        for i in 0..thread_nums {
            let name = format!("LuauApp-thread-{i}");

            std::thread::Builder::new()
                .name(name)
                .spawn_scoped(scope, || {
                    let mut closed_inner = closed.clone();

                    smol::block_on(executor.run(closed_inner.recv()))
                        .expect("Failed to run executor");
                })
                .expect("Failed to spawn thread");
        }
    });

    join.join().expect("Failed to join");
}
