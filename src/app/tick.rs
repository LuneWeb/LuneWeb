use super::AppProxy;
use mlua::ExternalResult;
use std::sync::Arc;

impl super::App {
    pub(super) async fn tick(
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
}
