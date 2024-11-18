use super::{Scheduler, Stopped, ALWAYS_SINGLE_THREAD};

fn initialize_tao(
    stopped: Stopped,
    send_proxy: async_broadcast::Sender<tao::event_loop::EventLoopProxy<crate::app::AppEvent>>,
) {
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    use tao::platform::unix::EventLoopBuilderExtUnix;

    #[cfg(target_os = "windows")]
    use tao::platform::windows::EventLoopBuilderExtWindows;

    let event_loop: tao::event_loop::EventLoop<crate::app::AppEvent> =
        tao::event_loop::EventLoopBuilder::with_user_event()
            .with_any_thread(true)
            .build();

    let proxy = event_loop.create_proxy();
    smol::block_on(send_proxy.broadcast(proxy)).expect("Failed to broadcast app proxy");

    let app_handle = crate::app::AppHandle {
        windows: Default::default(),
    };

    event_loop.run(move |event, _target, control_flow| match event {
        tao::event::Event::UserEvent(app_event) => {
            app_handle.process_app_event(app_event);
        }

        _ => {
            app_handle.process_tao_event(event, control_flow);

            if let tao::event_loop::ControlFlow::Exit = *control_flow {
                stopped.stop();
            }
        }
    })
}

pub fn initialize_threads(
    mut scheduler: Scheduler,
    f: impl FnOnce(tao::event_loop::EventLoopProxy<crate::app::AppEvent>) + Send + 'static,
) {
    let threads_count = std::thread::available_parallelism()
        .map_or(1, |x| x.get())
        .clamp(1, 8);

    if ALWAYS_SINGLE_THREAD {
        println!("[warn] ALWAYS_SINGLE_THREAD is set to true");
    }

    if threads_count == 1 || ALWAYS_SINGLE_THREAD {
        // single thread
        let stopped = scheduler.stopped.clone();

        scheduler
            .executor
            .spawn(async move {
                f(scheduler
                    .recv_proxy
                    .recv()
                    .await
                    .expect("Failed to receive proxy"));
            })
            .detach();

        std::thread::Builder::new()
            .name("user-thread".to_owned())
            .spawn(move || smol::block_on(scheduler.executor.run(scheduler.stopped.wait())))
            .expect("Failed to create thread");

        initialize_tao(stopped, scheduler.send_proxy);
    } else {
        // multi thread
        std::thread::scope(|scope| {
            std::thread::Builder::new()
                .name(format!("tao-thread"))
                .spawn_scoped(scope, || {
                    initialize_tao(scheduler.stopped.clone(), scheduler.send_proxy)
                })
                .expect("Failed to create thread");

            for i in 1..threads_count {
                let executor = &scheduler.executor;
                let stopped = &scheduler.stopped;

                std::thread::Builder::new()
                    .name(format!("user-thread-{i}"))
                    .spawn_scoped(scope, || smol::block_on(executor.run(stopped.wait())))
                    .expect("Failed to create thread");
            }

            f(smol::block_on(scheduler.recv_proxy.recv()).expect("Failed to receive proxy"));
        });
    }
}
