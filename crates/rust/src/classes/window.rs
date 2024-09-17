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

    pub fn new(lua: &mlua::Lua) -> Result<Self, String> {
        let Some(target) = lua.app_data_ref::<EventLoop>() else {
            return Err("Couldn't find reference to EventLoop, make sure to finalize EventLoop before attempting to create a Window".into());
        };

        let window = match Self::platform_specific().build(&target.inner) {
            Ok(window) => window,
            Err(err) => return Err(format!("Failed to create Window\nError: {err}")),
        };

        Ok(Self {
            inner: window,
            webview: None,
        })
    }

    pub fn with_webview<T: Fn(WebView) -> Result<WebView, String>>(
        mut self,
        lua: &mlua::Lua,
        dev: bool,
        webview_builder: T,
    ) -> Result<Self, String> {
        self.webview = Some(webview_builder(WebView::new(lua, &self, dev)?)?);
        Ok(self)
    }

    pub fn with_title(self, title: &str) -> Self {
        self.inner.set_title(title);
        self
    }

    pub fn finalize(self, lua: &Lua) -> Result<(), String> {
        let Some(mut target) = lua.app_data_mut::<EventLoop>() else {
            return Err("Couldn't find reference to EventLoop, make sure to finalize EventLoop before attempting to create a Window".into());
        };

        target.windows.push(self);

        Ok(())
    }
}
