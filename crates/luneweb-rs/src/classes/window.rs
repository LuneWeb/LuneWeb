use mlua::Lua;
use tao::window::{Window as _Window, WindowBuilder as _WindowBuilder};

use super::{eventloop::EventLoop, webview::WebView};

pub struct Window {
    pub inner: _Window,
    pub webview: Option<WebView>,
}

impl Window {
    fn platform_specific() -> _WindowBuilder {
        #[cfg(target_os = "linux")]
        {
            use tao::platform::unix::WindowBuilderExtUnix;
            _WindowBuilder::new().with_default_vbox(false)
        }

        #[cfg(not(target_os = "linux"))]
        _WindowBuilder::new()
    }

    pub fn new(lua: &mlua::Lua) -> Self {
        let target = lua
            .app_data_ref::<EventLoop>()
            .expect("Couldn't find reference to EventLoop, make sure to finalize EventLoop before attempting to create a Window");

        Self {
            inner: Self::platform_specific()
                .build(&target.inner)
                .expect("Failed to create Window"),
            webview: None,
        }
    }

    pub fn with_webview(mut self, webview_builder: fn(WebView) -> WebView) -> Self {
        self.webview = Some(webview_builder(WebView::new(&self)));
        self
    }

    pub fn with_title(self, title: &str) -> Self {
        self.inner.set_title(title);
        self
    }

    pub fn finalize(self, lua: &Lua) {
        let mut target = lua
            .app_data_mut::<EventLoop>()
            .expect("Couldn't find reference to EventLoop, make sure to finalize EventLoop before attempting to create a Window");

        target.windows.push(self)
    }
}
