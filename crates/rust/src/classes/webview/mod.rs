use std::{rc::Rc, sync::Mutex};

use wry::{http::Request, WebView as _WebView, WebViewBuilder as _WebViewBuilder};

use super::window::Window;

mod message;

pub struct WebView {
    pub inner: _WebView,
    pub message_pool: Rc<Mutex<Vec<String>>>,
}

const JS_IMPL: &str = include_str!(".js");

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

    pub fn new(target: &Window) -> Result<Self, String> {
        let pool_inner: Rc<Mutex<Vec<String>>> = Rc::new(Mutex::new(Vec::new()));
        let pool = Rc::clone(&pool_inner);

        let ipc = move |message: Request<String>| {
            pool_inner
                .lock()
                .expect("Failed to lock message pool")
                .insert(0, message.body().to_string());
        };

        let webview = match Self::platform_specific(target)
            .with_initialization_script(JS_IMPL)
            .with_ipc_handler(ipc)
            .with_devtools(true)
            .build()
        {
            Ok(webview) => webview,
            Err(err) => return Err(format!("Failed to create WebView\nError: {err}")),
        };

        Ok(Self {
            inner: webview,
            message_pool: pool,
        })
    }

    pub fn with_url(self, url: &str) -> Result<Self, String> {
        if let Err(err) = self.inner.load_url(url) {
            return Err(format!("Failed to load url '{url}'\n{err}"));
        }

        Ok(self)
    }

    pub fn with_dev(self, dev: bool) -> Result<Self, String> {
        if dev {
            self.inner.open_devtools();
        } else {
            self.inner.close_devtools();
        }

        Ok(self)
    }
}
