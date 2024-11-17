use std::collections::HashMap;

use mlua::ExternalResult;

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
struct App {
    windows: HashMap<tao::window::WindowId, tao::window::Window>,
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
        &self,
        event: tao::event::Event<'_, ()>,
        control_flow: &mut tao::event_loop::ControlFlow,
    ) {
        match event {
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
    }

    async fn run(mut self) -> mlua::Result<()> {
        let target = tao::event_loop::EventLoopBuilder::new()
            .with_any_thread(true)
            .build();

        let window = tao::window::WindowBuilder::new()
            .build(&target)
            .into_lua_err()?;

        self.windows.insert(window.id(), window);

        target.run(move |event, _, control_flow| {
            // runtime logic here
            smol::block_on(self.tick(event, control_flow));
        });
    }
}

fn main() {
    let app = App::default();
    let task = smol::spawn(app.run());

    smol::block_on(task).expect("Failed to run application");
}
