use crate::{Context, LuneWebError};
use mlua_luau_scheduler::Scheduler;
use std::{cell::RefCell, rc::Rc};
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
    ctx: Context,
    window: Option<Rc<Window>>,
    webview: Option<Rc<WebView>>,
}

impl App {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx,
            ..Default::default()
        }
    }

    fn build_window(&mut self, target: &EventLoop<()>) -> Result<Rc<Window>, LuneWebError> {
        let window = window_builder!().build(target)?;
        let rc = Rc::new(window);
        self.window = Some(Rc::clone(&rc));
        Ok(rc)
    }

    fn build_webview(&mut self) -> Result<Rc<WebView>, LuneWebError> {
        if let Some(window) = &self.window {
            let mut webview_builder = webview_builder!(window).with_url("about:blank");

            if let Some(javascript_dir) = &self.ctx.javascript {
                for file in javascript_dir.files() {
                    let Some(extension) = file.path().extension() else {
                        continue;
                    };

                    if extension == "js" {
                        let src = file
                            .contents_utf8()
                            .expect("Failed to interpret file's content as a string");

                        webview_builder = webview_builder.with_initialization_script(src);
                    }
                }
            }

            let rc = Rc::new(webview_builder.build()?);
            self.webview = Some(Rc::clone(&rc));
            Ok(rc)
        } else {
            Err("WebView should be built after Window".to_string().into())
        }
    }

    pub async fn run(mut self) -> Result<(), LuneWebError> {
        let window = EVENT_LOOP.with_borrow(|event_loop| self.build_window(event_loop))?;
        let lua = patched_lua!();
        let scheduler = Scheduler::new(&lua);
        self.build_webview()?;

        if let Some(luau_dir) = self.ctx.luau {
            let init = luau_dir
                .get_file("init.luau")
                .expect("Failed to get init.luau");

            let src = init
                .contents_utf8()
                .expect("Failed to interpret file's content as a string");

            let chunk = lua.load(src);
            scheduler.push_thread_front(chunk, ())?;
        }

        lune_std::inject_libraries(&mut self.ctx.lune_ctx)?;
        lua.sandbox(true)?; // R.I.P the _G global 💀
        lune_std::inject_globals(&lua, &self.ctx.lune_ctx.build())?;

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
                }

                Ok(())
            }
        })?;

        scheduler.push_thread_front(lua.create_thread(func).unwrap(), ())?;
        scheduler.run().await;

        Ok(())
    }
}
