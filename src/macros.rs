/// Create a window builder that has cross-platform support
#[macro_export]
macro_rules! window_builder {
    () => {{
        #[cfg(target_os = "linux")]
        {
            use tao::platform::unix::WindowBuilderExtUnix;
            use tao::window::WindowBuilder;
            WindowBuilder::new().with_default_vbox(false)
        }

        #[cfg(not(target_os = "linux"))]
        WindowBuilder::new()
    }};
}

/// Create a webview builder that has cross-platform support
#[macro_export]
macro_rules! webview_builder {
    ($target:expr) => {{
        #[cfg(not(target_os = "linux"))]
        {
            WebViewBuilder::new(target)
        }

        #[cfg(target_os = "linux")]
        {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilder;
            use wry::WebViewBuilderExtUnix;
            WebViewBuilder::new_gtk($target.gtk_window())
        }
    }};
}
