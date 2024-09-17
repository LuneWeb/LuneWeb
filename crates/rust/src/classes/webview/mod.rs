use super::window::Window;
use mlua::ExternalResult;
use std::{rc::Rc, sync::Mutex};
use wry::{http::Request, WebView as _WebView, WebViewBuilder as _WebViewBuilder};

mod message;

pub struct WebView {
    pub inner: _WebView,
    pub message_pool: Rc<Mutex<Vec<String>>>,
    pub dev_attached: bool,
}

#[derive(Debug, Default)]
pub struct Middlewares {
    vec: Vec<String>,
}

impl Middlewares {
    pub fn init(lua: &mlua::Lua) -> mlua::Result<()> {
        lua.set_app_data(Self::default());

        Self::add_middleware(lua, include_str!(".js"))
    }

    pub fn add_middleware(lua: &mlua::Lua, middleware: &str) -> mlua::Result<()> {
        let mut middlewares = lua.app_data_mut::<Self>().ok_or("Middlewares not found in lua app data container, make sure to call Middlewares::init method on the lua instance").into_lua_err()?;

        middlewares.vec.push(middleware.into());

        Ok(())
    }
}

impl WebView {
    fn platform_specific(target: &Window) -> _WebViewBuilder {
        #[cfg(target_os = "linux")]
        {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            _WebViewBuilder::new_gtk(target.inner.gtk_window())
        }

        #[cfg(not(target_os = "linux"))]
        {
            _WebViewBuilder::new(&target.inner)
        }
    }

    pub fn new(lua: &mlua::Lua, target: &Window, dev: bool) -> Result<Self, String> {
        let pool_inner: Rc<Mutex<Vec<String>>> = Rc::new(Mutex::new(Vec::new()));
        let pool = Rc::clone(&pool_inner);

        let ipc = move |message: Request<String>| {
            pool_inner
                .lock()
                .expect("Failed to lock message pool")
                .insert(0, message.body().to_string());
        };

        let middlewares = lua
            .app_data_ref::<Middlewares>()
            .ok_or("Middlewares not found in lua app data container, make sure to call Middlewares::init method on the lua instance")?;

        let mut webview_builder = Self::platform_specific(target)
            .with_ipc_handler(ipc)
            .with_devtools(dev);

        for middleware in middlewares.vec.iter() {
            webview_builder = webview_builder.with_initialization_script(middleware);
        }

        let webview = match webview_builder.build() {
            Ok(webview) => webview,
            Err(err) => return Err(format!("Failed to create WebView\nError: {err}")),
        };

        Ok(Self {
            inner: webview,
            message_pool: pool,
            dev_attached: dev,
        })
    }

    pub fn with_url(self, url: &str) -> Result<Self, String> {
        if let Err(err) = self.inner.load_url(url) {
            return Err(format!("Failed to load url '{url}'\n{err}"));
        }

        Ok(self)
    }

    pub fn toggle_dev(&self, dev: bool) {
        if dev & !self.dev_attached {
            println!("[Warn] tried to toggle dev tools but webview doesn't have dev tools attached to it");
        }

        if dev {
            self.inner.open_devtools();
        } else {
            self.inner.close_devtools();
        }
    }
}
