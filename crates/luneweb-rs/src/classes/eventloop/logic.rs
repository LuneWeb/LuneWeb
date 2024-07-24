use std::time::Duration;

use tao::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    platform::run_return::EventLoopExtRunReturn,
    window::WindowId,
};

use super::EventLoop;

/// NOTE: increase the duration if the application crashes regularly
const INTERVAL: Duration = Duration::from_millis(4);

pub enum EventLoopAction {
    CloseRequested(WindowId),
    RedrawRequest(WindowId),
    None,
}

pub async fn lua_run(lua: &mlua::Lua, _: ()) -> mlua::Result<()> {
    loop {
        let Some(mut event_loop) = lua.app_data_mut::<EventLoop>() else {
            continue;
        };

        event_loop.once();

        if !event_loop.windows.is_empty()
            && event_loop
                .windows
                .iter()
                .all(|window| !window.inner.is_visible())
        {
            // All windows are closed
            event_loop.once();
            break;
        }

        // drop mutable reference before using await
        drop(event_loop);

        tokio::time::sleep(INTERVAL).await;
    }

    std::process::exit(0)
}

impl EventLoop {
    fn await_action(&mut self) -> EventLoopAction {
        let mut action = EventLoopAction::None;

        self.inner.run_return(|event, _target, control_flow| {
            if can_exit(&event) {
                *control_flow = ControlFlow::Exit;
            }

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::CloseRequested,
                    ..
                } => action = EventLoopAction::CloseRequested(window_id),

                Event::RedrawRequested(window_id) => {
                    action = EventLoopAction::RedrawRequest(window_id)
                }

                _ => {}
            };
        });

        action
    }

    fn take_action(&self, action: &EventLoopAction) {
        match action {
            EventLoopAction::None => {}

            EventLoopAction::RedrawRequest(window_id) => {
                if let Some(window) = self.get_window(*window_id) {
                    window.inner.request_redraw();
                }
            }

            EventLoopAction::CloseRequested(window_id) => {
                if let Some(window) = self.get_window(*window_id) {
                    window.inner.set_visible(false);
                }
            }
        }
    }

    pub fn once(&mut self) {
        let action = self.await_action();

        self.take_action(&action);
    }
}

fn can_exit(event: &Event<()>) -> bool {
    matches!(
        event,
        tao::event::Event::MainEventsCleared
            | tao::event::Event::LoopDestroyed
            | tao::event::Event::WindowEvent { .. }
            | tao::event::Event::UserEvent(_)
    )
}
