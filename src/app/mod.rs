use crate::{Context, LuneWebError};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use wry::WebView;

#[macro_use]
mod util;

#[derive(Default)]
pub struct App<'a> {
    ctx: Context<'a>,
    webview: Option<WebView>,
}

impl<'app> App<'app> {
    pub fn new(ctx: Context<'app>) -> Self {
        Self {
            ctx,
            ..Default::default()
        }
    }

    pub fn run(&mut self) -> Result<(), LuneWebError> {
        let event_loop = EventLoop::new();
        let window = window_builder!().build(&event_loop)?;

        let mut webview_builder = webview_builder!(&window).with_url("about:blank");

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

        self.webview = Some(webview_builder.build()?);

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
