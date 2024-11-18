use std::{collections::HashMap, sync::Arc};

pub mod proxy;
mod tick;

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

#[derive(Debug, Default)]
pub struct App {
    windows: HashMap<tao::window::WindowId, Arc<tao::window::Window>>,
}

impl App {
    fn should_exit(&self) -> bool {
        for window in self.windows.values() {
            if window.is_visible() {
                return false;
            }
        }

        true
    }

    pub async fn run(
        mut self,
    ) -> (
        tao::event_loop::EventLoopProxy<proxy::AppProxy>,
        std::thread::JoinHandle<()>,
    ) {
        let (send_proxy, receive_proxy) = flume::unbounded();
        let join = std::thread::Builder::new()
            .name("LuauApp".to_string())
            .spawn(move || {
                let target: tao::event_loop::EventLoop<proxy::AppProxy> =
                    tao::event_loop::EventLoopBuilder::with_user_event()
                        .with_any_thread(true)
                        .build();

                send_proxy
                    .send(target.create_proxy())
                    .expect("Failed to send proxy to main thread");

                target.run(move |event, target, control_flow| {
                    // runtime logic here
                    smol::block_on(self.process_event(event, target, control_flow))
                        .expect("Application tick failed");
                });
            })
            .expect("Failed to spawn a thread for application");

        let proxy = receive_proxy
            .recv_async()
            .await
            .expect("Failed to receive proxy from application thread");

        (proxy, join)
    }
}
