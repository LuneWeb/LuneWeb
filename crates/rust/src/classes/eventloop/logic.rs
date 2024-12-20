use super::EventLoop;
use std::collections::VecDeque;
use tao::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    platform::run_return::EventLoopExtRunReturn,
    window::WindowId,
};

pub enum EventLoopAction {
    CloseRequested(WindowId),
    RedrawRequest(WindowId),
}

impl EventLoop {
    fn await_actions(&mut self) -> VecDeque<EventLoopAction> {
        let mut actions: VecDeque<EventLoopAction> = VecDeque::new();

        self.inner.run_return(|event, _target, control_flow| {
            if can_exit(&event) {
                *control_flow = ControlFlow::Exit;
            }

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::CloseRequested,
                    ..
                } => actions.push_back(EventLoopAction::CloseRequested(window_id)),

                Event::RedrawRequested(window_id) => {
                    actions.push_back(EventLoopAction::RedrawRequest(window_id));
                }

                _ => {}
            };
        });

        actions
    }

    fn take_action(&self, action: &EventLoopAction) {
        match action {
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
        for action in self.await_actions() {
            self.take_action(&action);
        }
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
