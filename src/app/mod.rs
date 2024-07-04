use crate::{Context, LuneWebError};
use std::rc::Rc;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use wry::WebView;

#[macro_use]
mod util;

#[derive(Default)]
pub struct App<'a> {
    ctx: Context<'a>,
    window: Option<Rc<Window>>,
    webview: Option<Rc<WebView>>,
}

impl<'app> App<'app> {
    pub fn new(ctx: Context<'app>) -> Self {
        Self {
            ctx,
            ..Default::default()
        }
    }

    fn build_event_loop(&mut self) -> EventLoop<()> {
        EventLoop::new()
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

    pub fn run(&mut self) -> Result<(), LuneWebError> {
        let event_loop = self.build_event_loop();
        let window = self.build_window(&event_loop)?;

        self.build_webview()?;

        event_loop.run(move |event, _target, control_flow| match event {
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::CloseRequested,
                ..
            } => {
                window.set_visible(false);
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                window.request_redraw();
            }
            _ => {}
        });
    }
}
