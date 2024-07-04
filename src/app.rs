use crate::{Context, LuneWebError};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

/// Create a window builder that has cross-platform support
macro_rules! window_builder {
    () => {{
        #[cfg(target_os = "linux")]
        {
            use tao::platform::unix::WindowBuilderExtUnix;
            WindowBuilder::new().with_default_vbox(false)
        }

        #[cfg(not(target_os = "linux"))]
        WindowBuilder::new()
    }};
}

/// Create a webview builder that has cross-platform support
macro_rules! webview_builder {
    ($target:expr) => {{
        #[cfg(not(target_os = "linux"))]
        {
            WebViewBuilder::new(target)
        }

        #[cfg(target_os = "linux")]
        {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            WebViewBuilder::new_gtk($target.gtk_window())
        }
    }};
}

pub struct App<'a> {
    ctx: Context<'a>,
}

impl<'app> App<'app> {
    pub fn new(ctx: Context<'app>) -> Self {
        Self { ctx }
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

            webview_builder!(&window)
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
