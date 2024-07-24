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
            WebViewBuilder::new(target.inner)
        }
    }

    pub fn new(target: &Window) -> Self {
        Self {
            inner: Self::platform_specific(target)
                .build()
                .expect("Failed to create WebView"),
        }
    }
}
