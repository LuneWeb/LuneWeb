use std::collections::HashMap;
use tao::window::{Window, WindowId};

#[derive(Debug, Clone)]
pub enum AppEvent {}

#[derive(Debug)]
pub(crate) struct AppHandle {
    pub(crate) windows: HashMap<WindowId, Window>,
}

impl AppHandle {
    pub(crate) fn process_app_event(&self, event: AppEvent) {}

    pub(crate) fn process_tao_event(
        &self,
        event: tao::event::Event<AppEvent>,
        control_flow: &mut tao::event_loop::ControlFlow,
    ) {
        *control_flow = tao::event_loop::ControlFlow::Exit;
    }
}
