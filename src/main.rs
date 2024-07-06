use std::{cell::RefCell, process::exit as process_exit, rc::Rc, time::Duration};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
};

#[macro_use]
mod macros;
mod cli;
mod config;

thread_local! {
    pub static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoopBuilder::with_user_event().build());
}

#[tokio::main]
async fn main() {
    cli::init().await
}

pub async fn logic(window: Rc<Window>) -> mlua::Result<()> {
    loop {
        let mut exit = false;

        EVENT_LOOP.with_borrow_mut(|event_loop| {
            event_loop.run_return(|event, _target, control_flow| {
                match event {
                    Event::WindowEvent {
                        window_id: _,
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        window.set_visible(false);
                        exit = true;
                    }
                    Event::RedrawRequested(_) => {
                        window.request_redraw();
                    }
                    _ => {}
                };

                let can_exit = matches!(
                    event,
                    tao::event::Event::MainEventsCleared
                        | tao::event::Event::LoopDestroyed
                        | tao::event::Event::WindowEvent { .. }
                        | tao::event::Event::UserEvent(_)
                );

                if can_exit {
                    *control_flow = ControlFlow::Exit;
                }
            });
        });

        if exit {
            break;
        }

        tokio::time::sleep(Duration::from_millis(16)).await;
    }

    process_exit(0);
}
