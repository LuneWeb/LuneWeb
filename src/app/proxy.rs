use mlua::ExternalResult;
use std::sync::Arc;

#[derive(Debug)]
pub enum AppProxy {
    CreateWindow {
        send_window: flume::Sender<Arc<tao::window::Window>>,
    },
}

impl AppProxy {
    pub async fn create_window(
        proxy: tao::event_loop::EventLoopProxy<Self>,
    ) -> mlua::Result<Arc<tao::window::Window>> {
        let (sender, receiver) = flume::unbounded();

        proxy
            .send_event(Self::CreateWindow {
                send_window: sender,
            })
            .into_lua_err()?;

        receiver.recv().into_lua_err()
    }
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
