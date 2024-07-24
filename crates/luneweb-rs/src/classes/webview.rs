use wry::{WebView as _WebView, WebViewBuilder as _WebViewBuilder};

use super::window::Window;

pub struct WebView {
    pub inner: _WebView,
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
            _WebViewBuilder::new(target.inner)
        }
    }

    pub fn new(target: &Window) -> Result<Self, String> {
        let webview = match Self::platform_specific(target).build() {
            Ok(webview) => webview,
            Err(err) => return Err(format!("Failed to create WebView\nError: {err}")),
        };

        Ok(Self { inner: webview })
    }

    pub fn with_url(self, url: &str) -> Result<Self, String> {
        if let Err(err) = self.inner.load_url(url) {
            return Err(format!("Failed to load url '{url}'\n{err}"));
        }

        Ok(self)
    }
}
