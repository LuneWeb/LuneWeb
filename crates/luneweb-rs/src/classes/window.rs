use mlua::Lua;
use tao::window::{Window as _Window, WindowBuilder as _WindowBuilder};

use super::{eventloop::EventLoop, webview::WebView};

pub struct Window {
    pub inner: _Window,
    pub webview: Option<WebView>,
}

impl Window {
    pub fn new(lua: &mlua::Lua) -> Self {
        let target = lua
            .app_data_ref::<EventLoop>()
            .expect("Couldn't find reference to EventLoop, make sure to finalize EventLoop before attempting to create a Window");

        Self {
            inner: _WindowBuilder::new()
                .build(&target.inner)
                .expect("Failed to create Window"),
            webview: None,
        }
    }

    pub fn with_webview(mut self) -> Self {
        self.webview = Some(WebView::new(&self));
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
