use mlua::ExternalResult;
use std::sync::Arc;

#[derive(Debug)]
pub enum AppProxy {
    CreateWindow {
        send_window: flume::Sender<Arc<tao::window::Window>>,
    },
}

impl super::App {
    pub(super) async fn process_proxy(
        &mut self,
        proxy: AppProxy,
        target: &tao::event_loop::EventLoopWindowTarget<AppProxy>,
    ) -> mlua::Result<()> {
        match proxy {
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
        }

        Ok(())
    }
}
