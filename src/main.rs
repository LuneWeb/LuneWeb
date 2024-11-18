use mlua::ExternalResult;
use std::{collections::HashMap, sync::Arc};

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

#[derive(Debug)]
pub enum AppProxy {
    CreateWindow {
        send_window: flume::Sender<Arc<tao::window::Window>>,
    },
}

#[derive(Debug, Default)]
struct App {
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

    async fn tick(
        &mut self,
        target_event: tao::event::Event<'_, AppProxy>,
        target: &tao::event_loop::EventLoopWindowTarget<AppProxy>,
        control_flow: &mut tao::event_loop::ControlFlow,
    ) -> mlua::Result<()> {
        match target_event {
            tao::event::Event::UserEvent(proxy) => match proxy {
                AppProxy::CreateWindow { send_window } => {
                    let window = Arc::new(
                        tao::window::WindowBuilder::new()
                            .build(&target)
                            .into_lua_err()?,
                    );

                    send_window
                        .send_async(Arc::clone(&window))
                        .await
                        .into_lua_err()?;

                    self.windows.insert(window.id(), window);
                }
            },

            tao::event::Event::RedrawRequested(id) => {
                if let Some(window) = self.windows.get(&id) {
                    window.request_redraw();
                }
            }

            tao::event::Event::WindowEvent {
                event: window_event,
                window_id: id,
                ..
            } => match window_event {
                tao::event::WindowEvent::CloseRequested => {
                    if let Some(window) = self.windows.get(&id) {
                        window.set_visible(false);
                    }

                    if self.should_exit() {
                        *control_flow = tao::event_loop::ControlFlow::Exit;
                    }
                }

                _ => {}
            },

            _ => {}
        }

        Ok(())
    }

    async fn run(
        mut self,
    ) -> (
        tao::event_loop::EventLoopProxy<AppProxy>,
        std::thread::JoinHandle<()>,
    ) {
        let (send_proxy, receive_proxy) = flume::unbounded();
        let join = std::thread::Builder::new()
            .name("LuauApp".to_string())
            .spawn(move || {
                let target: tao::event_loop::EventLoop<AppProxy> =
                    tao::event_loop::EventLoopBuilder::with_user_event()
                        .with_any_thread(true)
                        .build();

                send_proxy
                    .send(target.create_proxy())
                    .expect("Failed to send proxy to main thread");

                target.run(move |event, target, control_flow| {
                    // runtime logic here
                    smol::block_on(self.tick(event, target, control_flow))
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

fn main() {
    let app = App::default();
    let (proxy, join) = smol::block_on(app.run());

    smol::spawn(async move {
        let (sender, receiver) = flume::unbounded();

        proxy
            .send_event(AppProxy::CreateWindow {
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
