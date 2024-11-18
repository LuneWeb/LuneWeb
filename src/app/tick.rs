use super::proxy::AppProxy;

impl super::App {
    pub(super) async fn process_event(
        &mut self,
        target_event: tao::event::Event<'_, AppProxy>,
        target: &tao::event_loop::EventLoopWindowTarget<AppProxy>,
        control_flow: &mut tao::event_loop::ControlFlow,
    ) -> mlua::Result<()> {
        match target_event {
            tao::event::Event::UserEvent(proxy) => self.process_proxy(proxy, target).await?,

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
                        self.closed
                            .0
                            .broadcast(())
                            .await
                            .expect("Failed to broadcast closing signal");

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