use std::{collections::HashMap, sync::Arc};
use tao::window::{Window, WindowBuilder, WindowId};

#[derive(Debug, Clone)]
pub enum AppEvent {
    CreateWindow(flume::Sender<Arc<Window>>),
}

#[derive(Debug, Clone)]
pub struct AppProxy {
    pub(crate) proxy: tao::event_loop::EventLoopProxy<AppEvent>,
}

impl AppProxy {
    pub fn create_window(&self) -> Arc<Window> {
        let (sender, receiver) = flume::bounded(1);
        self.proxy
            .send_event(AppEvent::CreateWindow(sender))
            .expect("Failed to send event");
        receiver.recv().expect("Failed to receive window")
    }
}

#[derive(Debug)]
pub(crate) struct AppHandle {
    pub(crate) windows: HashMap<WindowId, Arc<Window>>,
}

impl AppHandle {
    pub(crate) async fn process_app_event(
        &mut self,
        event: AppEvent,
        target: &tao::event_loop::EventLoopWindowTarget<AppEvent>,
    ) {
        match event {
            AppEvent::CreateWindow(sender) => {
                let window = Arc::new(
                    WindowBuilder::new()
                        .with_title("LuauApp")
                        .build(&target)
                        .expect("Failed to create window"),
                );

                sender
                    .send_async(Arc::clone(&window))
                    .await
                    .expect("Failed to send window");

                self.windows.insert(window.id(), window);
            }
        }
    }

    pub(crate) async fn process_tao_event(
        &mut self,
        event: tao::event::Event<'_, AppEvent>,
        target: &tao::event_loop::EventLoopWindowTarget<AppEvent>,
        control_flow: &mut tao::event_loop::ControlFlow,
    ) {
        match event {
            tao::event::Event::RedrawRequested(id) => {
                if let Some(window) = self.windows.get(&id) {
                    window.request_redraw();
                }
            }

            tao::event::Event::WindowEvent {
                window_id: id,
                event: tao::event::WindowEvent::CloseRequested,
                ..
            } => {
                if let Some(window) = self.windows.get(&id) {
                    window.set_visible(false);
                }

                if self.windows.values().all(|x| !x.is_visible()) {
                    *control_flow = tao::event_loop::ControlFlow::Exit;
                }
            }

            _ => {}
        }
    }
}
