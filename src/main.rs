use std::{cell::RefCell, rc::Rc, time::Duration};

use include_dir::{include_dir, Dir};
use tokio::sync::watch::Sender;

use lune_std::context::GlobalsContextBuilder;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
};
use wry::WebView;

#[macro_use]
mod macros;
mod cli;
mod config;
mod message;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const LUAU_TYPES: Dir = include_dir!("types/");

#[derive(Default)]
struct App {
    pub webview: Option<Rc<WebView>>,
}

thread_local! {
    pub static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoopBuilder::with_user_event().build());
    pub static APP: RefCell<App> = RefCell::new(App::default());
    pub static ONLOAD_TX: RefCell<Sender<()>> = RefCell::new(Sender::new(()));
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    cli::init().await
}

pub fn inject_libraries(ctx: &mut GlobalsContextBuilder) -> mlua::Result<()> {
    ctx.with_alias("luneweb", |libs| {
        libs.insert(
            "message",
            lune_std::context::LuauLibraryCreator::LuaTable(message::create),
        );

        Ok(())
    })
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

    Ok(())
}
