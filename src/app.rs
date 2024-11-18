use std::{collections::HashMap, sync::Arc};
use tao::window::{Window, WindowBuilder, WindowId};

#[derive(Debug, Clone)]
pub enum AppEvent {
    CreateWindow {
        sender: flume::Sender<Arc<Window>>,
        title: Option<String>,
    },
    SpawnLuaThread {
        thread: mlua::Thread,
    },
}

#[derive(Debug, Clone)]
pub struct AppProxy {
    pub(crate) proxy: tao::event_loop::EventLoopProxy<AppEvent>,
}

impl AppProxy {
    pub fn create_window(&self, title: Option<String>) -> Arc<Window> {
        let (sender, receiver) = flume::bounded(1);
        self.proxy
            .send_event(AppEvent::CreateWindow { sender, title })
            .expect("Failed to send event");
        receiver.recv().expect("Failed to receive window")
    }

    pub fn spawn_lua_thread(&self, thread: mlua::Thread) {
        self.proxy
            .send_event(AppEvent::SpawnLuaThread { thread })
            .expect("Failed to send event");
    }
}

#[derive(Debug, Default)]
pub(crate) struct AppHandle {
    pub(crate) windows: HashMap<WindowId, Arc<Window>>,
    pub(crate) lua_threads: Vec<mlua::Thread>,
}

impl AppHandle {
    pub(crate) async fn process(&mut self) {
        self.lua_threads = self
            .lua_threads
            .drain(..)
            .filter(|thread| crate::scheduler::thread::process_lua_thread(thread, None))
            .collect();
        self.lua_threads.shrink_to_fit();
    }

    pub(crate) async fn process_app_event(
        &mut self,
        event: AppEvent,
        target: &tao::event_loop::EventLoopWindowTarget<AppEvent>,
    ) {
        match event {
            AppEvent::CreateWindow { sender, title } => {
                let window = Arc::new(
                    WindowBuilder::new()
                        .with_title(title.unwrap_or(crate::WINDOW_DEFAULT_TITLE.to_owned()))
                        .build(&target)
                        .expect("Failed to create window"),
                );

                sender
                    .send_async(Arc::clone(&window))
                    .await
                    .expect("Failed to send window");

                self.windows.insert(window.id(), window);
            }

            AppEvent::SpawnLuaThread { thread } => {
                self.lua_threads.push(thread);
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
