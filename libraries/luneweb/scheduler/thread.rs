use super::{Scheduler, Stopped};
use crate::ALWAYS_SINGLE_THREAD;
use smol::Task;
use std::future::Future;

pub fn process_lua_thread(
    thread: &mlua::Thread,
    args: Option<mlua::MultiValue>,
) -> Option<mlua::Result<mlua::MultiValue>> {
    match thread.resume::<mlua::MultiValue>(args.unwrap_or_default()) {
        Ok(v) => {
            if v.get(0).is_some_and(|x| {
                x.as_light_userdata()
                    .is_some_and(|x| x == mlua::Lua::poll_pending())
            }) {
                None
            } else {
                Some(Ok(v))
            }
        }
        Err(err) => {
            eprintln!("{err}");

            Some(Err(err))
        }
    }
}

fn initialize_tao(stopped: Stopped, send_proxy: async_broadcast::Sender<crate::app::AppProxy>) {
    let event_loop: tao::event_loop::EventLoop<crate::app::AppEvent> =
        tao::event_loop::EventLoopBuilder::with_user_event().build();

    let proxy = event_loop.create_proxy();
    smol::block_on(send_proxy.broadcast(crate::app::AppProxy { proxy }))
        .expect("Failed to broadcast app proxy");

    let mut app_handle = crate::app::AppHandle::default();

    event_loop.run(move |event, target, control_flow| {
        match event {
            tao::event::Event::UserEvent(app_event) => {
                smol::block_on(app_handle.process_app_event(app_event, target));
            }

            _ => {
                smol::block_on(app_handle.process_tao_event(event, target, control_flow));

                if let tao::event_loop::ControlFlow::Exit = *control_flow {
                    stopped.stop();
                }
            }
        }

        smol::block_on(app_handle.process());
    })
}

pub trait LuaThreadMethods {
    fn spawn<T: Send + 'static>(&self, future: impl Future<Output = T> + Send + 'static)
        -> Task<T>;
}

impl LuaThreadMethods for mlua::Lua {
    fn spawn<T: Send + 'static>(
        &self,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Task<T> {
        let scheduler = self.app_data_ref::<Scheduler>().expect("Failed to find scheduler in app data container, have you trued calling initialize_threads on this lua vm?");

        scheduler.executor.spawn(future)
    }
}

pub fn initialize_threads(lua: mlua::Lua, f: impl FnOnce(crate::app::AppProxy) + Send + 'static) {
    let scheduler = Scheduler::new();
    let (proxy_sender, mut proxy_receiver) = async_broadcast::broadcast(1);

    lua.set_app_data(scheduler.clone());

    let threads_count = std::thread::available_parallelism()
        .map_or(1, |x| x.get())
        .clamp(1, 8);

    if ALWAYS_SINGLE_THREAD {
        println!("[warn] ALWAYS_SINGLE_THREAD is set to true");
    }

    let stopped = scheduler.stopped.clone();

    scheduler
        .executor
        .spawn(async move {
            f(smol::block_on(proxy_receiver.recv()).expect("Failed to receive proxy"));
        })
        .detach();

    // smol executor thread
    if threads_count == 1 || ALWAYS_SINGLE_THREAD {
        // single threaded
        std::thread::Builder::new()
            .name(format!("executor-0"))
            .spawn(move || smol::block_on(scheduler.executor.run(scheduler.stopped.wait())))
            .expect("Failed to create thread");
    } else {
        // multi threaded
        for i in 0..threads_count {
            let executor = scheduler.executor.clone();
            let stopped = scheduler.stopped.clone();

            std::thread::Builder::new()
                .name(format!("executor-{i}"))
                .spawn(move || smol::block_on(executor.run(stopped.wait())))
                .expect("Failed to create thread");
        }
    }

    initialize_tao(stopped, proxy_sender);
}
