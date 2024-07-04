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

        if let Some(javascript_dir) = &self.ctx.javascript {
            let index = javascript_dir
                .get_file("index.js")
                .expect("Failed to find index.js");

            let src = index
                .contents_utf8()
                .expect("Failed to interpret file's content as a string");

            self.webview = Some(
                webview_builder!(&window)
                    .with_initialization_script(src)
                    .with_url("about:blank")
                    .build()?,
            )
        }

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
