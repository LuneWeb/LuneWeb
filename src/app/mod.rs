use crate::{ctx::ContextBuilder, Context, LuneWebError};
use mlua_luau_scheduler::Scheduler;
use std::{cell::RefCell, rc::Rc, time::Duration};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::run_return::EventLoopExtRunReturn,
    window::Window,
};
use wry::WebView;

#[macro_use]
mod util;

thread_local! {
    pub static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoopBuilder::with_user_event().build());
}

#[derive(Default)]
pub struct App {
    pub(crate) ctx: Context,
    pub(crate) window: Option<Rc<Window>>,
    pub(crate) webview: Option<Rc<WebView>>,
}

impl App {
    fn build_window(&mut self) -> Result<Rc<Window>, LuneWebError> {
        let window = EVENT_LOOP.with_borrow(|event_loop| window_builder!().build(event_loop))?;
        let rc = Rc::new(window);
        self.window = Some(Rc::clone(&rc));

        Ok(rc)
    }

    fn build_webview(&mut self) -> Result<Rc<WebView>, LuneWebError> {
        if let Some(window) = &self.window {
            let builder = webview_builder!(window).with_url("about:blank");
            let rc = Rc::new(builder.build()?);
            self.webview = Some(Rc::clone(&rc));
            Ok(rc)
        } else {
            Err("WebView should be built after Window".to_string().into())
        }
    }

    pub async fn run(mut self) -> Result<(), LuneWebError> {
        let lua = patched_lua!(&self.ctx.lune_ctx);
        let scheduler = Scheduler::new(&lua);

        let window = self.build_window()?;
        self.build_webview()?;

        let func = lua.create_async_function(move |_, _: ()| {
            let window = Rc::clone(&window);

            async move {
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
                        })
                    });

                    if exit {
                        break;
                    }

                    tokio::time::sleep(Duration::from_millis(16)).await;
                }

                Ok(())
            }
        })?;
        scheduler.push_thread_front(lua.create_thread(func)?, ())?;
        scheduler.run().await;

        Ok(())
    }
}

impl From<Context> for App {
    fn from(value: Context) -> Self {
        Self {
            ctx: value,
            ..Default::default()
        }
    }
}

impl From<ContextBuilder> for App {
    fn from(value: ContextBuilder) -> Self {
        let ctx: Context = value.into();
        Self::from(ctx)
    }
}