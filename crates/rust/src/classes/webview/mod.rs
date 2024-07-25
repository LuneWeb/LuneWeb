use tokio::sync::watch;
use wry::{http::Request, WebView as _WebView, WebViewBuilder as _WebViewBuilder};

use super::window::Window;

mod message;

pub struct WebView {
    pub inner: _WebView,
    pub messages: watch::Receiver<String>,
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
            _WebViewBuilder::new(target.inner)
        }
    }

    pub fn new(target: &Window) -> Result<Self, String> {
        let (tx, rx) = watch::channel(String::from("undefined"));
        let ipc = move |message: Request<String>| {
            let _ = tx.send(message.into_body());
        };

        let webview = match Self::platform_specific(target)
            .with_initialization_script(JS_IMPL)
            .with_ipc_handler(ipc)
            .build()
        {
            Ok(webview) => webview,
            Err(err) => return Err(format!("Failed to create WebView\nError: {err}")),
        };

        Ok(Self {
            inner: webview,
            messages: rx,
        })
    }

    pub fn with_url(self, url: &str) -> Result<Self, String> {
        if let Err(err) = self.inner.load_url(url) {
            return Err(format!("Failed to load url '{url}'\n{err}"));
        }

        Ok(self)
    }
}
