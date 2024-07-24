use std::time::Duration;

use tao::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    platform::run_return::EventLoopExtRunReturn,
    window::WindowId,
};

use crate::classes::window::Window;

use super::EventLoop;

pub enum EventLoopAction {
    CloseRequested(WindowId),
    RedrawRequest(WindowId),
    None,
}

pub async fn lua_run(lua: &mlua::Lua, _: ()) -> mlua::Result<()> {
    let can_exit = false;

    loop {
        tokio::time::sleep(Duration::from_millis(16)).await;

        let Some(mut event_loop) = lua.app_data_mut::<EventLoop>() else {
            continue;
        };

        event_loop.once();

        if can_exit {
            break;
        }
    }

    Ok(())
}

impl EventLoop {
    fn await_action(&mut self) -> EventLoopAction {
        let mut action = EventLoopAction::None;

        self.inner.run_return(|event, _target, control_flow| {
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

            if can_exit(event) {
                *control_flow = ControlFlow::Exit;
            }
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

    fn get_window(&self, window_id: WindowId) -> Option<&Window> {
        self.windows
            .iter()
            .find(|window| window.inner.id() == window_id)
    }
}

fn can_exit(event: Event<()>) -> bool {
    matches!(
        event,
        tao::event::Event::MainEventsCleared
            | tao::event::Event::LoopDestroyed
            | tao::event::Event::WindowEvent { .. }
            | tao::event::Event::UserEvent(_)
    )
}
