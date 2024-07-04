use crate::{Context, LuneWebError};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

pub struct App<'a> {
    ctx: Context<'a>,
}

impl<'app> App<'app> {
    pub fn new(ctx: Context<'app>) -> Self {
        Self { ctx }
    }

    pub fn run(&mut self) -> Result<(), LuneWebError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop)?;

        if let Some(javascript_dir) = &self.ctx.javascript {
            let index = javascript_dir
                .get_file("index.js")
                .expect("Failed to find index.js");

            let src = index
                .contents_utf8()
                .expect("Failed to interpret file's content as a string");

            WebViewBuilder::new(&window)
                .with_initialization_script(src)
                .build()?;
        }

        event_loop.run(move |event, _target, control_flow| {
            if let Event::WindowEvent {
                window_id: _,
                event: WindowEvent::CloseRequested,
                ..
            } = event
            {
                window.set_visible(false);
                *control_flow = ControlFlow::Exit;
            }
        });
    }
}
