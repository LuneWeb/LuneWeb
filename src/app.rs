use mlua::ExternalResult;
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
        args: Option<mlua::MultiValue>,
    },
    AwaitLuaThread {
        sender: flume::Sender<mlua::Result<mlua::MultiValue>>,
        thread: mlua::Thread,
    },
}

#[derive(Debug, Clone)]
pub struct AppProxy {
    pub(crate) proxy: tao::event_loop::EventLoopProxy<AppEvent>,
}

impl AppProxy {
    pub async fn create_window(&self, title: Option<String>) -> Arc<Window> {
        let (sender, receiver) = flume::bounded(1);
        self.proxy
            .send_event(AppEvent::CreateWindow { sender, title })
            .expect("Failed to send event");
        receiver
            .recv_async()
            .await
            .expect("Failed to receive window")
    }

    pub fn spawn_lua_thread(&self, thread: mlua::Thread, args: Option<mlua::MultiValue>) {
        self.proxy
            .send_event(AppEvent::SpawnLuaThread { thread, args })
            .expect("Failed to send event");
    }

    pub async fn await_lua_thread(&self, thread: mlua::Thread) -> mlua::Result<mlua::MultiValue> {
        let (sender, receiver) = flume::bounded(1);
        self.proxy
            .send_event(AppEvent::AwaitLuaThread { sender, thread })
            .into_lua_err()?;
        receiver.recv_async().await.into_lua_err()?
    }
}

#[derive(Default)]
pub(crate) struct AppHandle {
    pub(crate) windows: HashMap<WindowId, Arc<Window>>,
    pub(crate) lua_threads: Vec<(mlua::Thread, mlua::MultiValue)>,
    pub(crate) lua_thread_listeners:
        HashMap<usize, Vec<flume::Sender<mlua::Result<mlua::MultiValue>>>>,
}

impl AppHandle {
    pub(crate) async fn process(&mut self) {
        let mut cleanup: Vec<usize> = Vec::new();

        for (thread, args) in &self.lua_threads {
            if let Some(result) =
                crate::scheduler::thread::process_lua_thread(thread, Some(args.to_owned()))
            {
                let thread_id = thread.to_pointer() as usize;

                if let Some(listeners) = self.lua_thread_listeners.get(&thread_id) {
                    for sender in listeners {
                        sender
                            .send_async(result.clone())
                            .await
                            .expect("Failed to send lua thread result");
                    }
                }

                cleanup.push(thread_id);
            }
        }

        self.lua_threads = self
            .lua_threads
            .drain(..)
            .filter(|x| !cleanup.contains(&(x.0.to_pointer() as usize)))
            .collect();
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

            AppEvent::SpawnLuaThread { thread, args } => {
                self.lua_threads.push((thread, args.unwrap_or_default()));
            }

            AppEvent::AwaitLuaThread { sender, thread } => {
                let thread_id = thread.to_pointer() as usize;

                if let Some(listeners) = self.lua_thread_listeners.get_mut(&thread_id) {
                    listeners.push(sender);
                } else {
                    self.lua_thread_listeners.insert(thread_id, vec![sender]);
                }
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
            }

            _ => {}
        }

        if !self.windows.is_empty() {
            if self.windows.values().all(|x| !x.is_visible()) {
                *control_flow = tao::event_loop::ControlFlow::Exit;
            }
        }
    }
}
