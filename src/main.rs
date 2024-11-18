mod app;

fn main() {
    let app = app::App::default();
    let (proxy, join) = smol::block_on(app.run());

    smol::spawn(async move {
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

    join.join().expect("Failed to join");
}
