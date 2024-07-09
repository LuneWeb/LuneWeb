/// Create a window builder that has cross-platform support
#[macro_export]
macro_rules! window_builder {
    () => {{
        use tao::window::WindowBuilder;

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
#[macro_export]
macro_rules! webview_builder {
    ($target:expr) => {{
        use wry::WebViewBuilder;

        #[cfg(target_os = "linux")]
        {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            WebViewBuilder::new_gtk($target.gtk_window())
        }

        #[cfg(not(target_os = "linux"))]
        {
            WebViewBuilder::new(&$target)
        }
    }};
}

macro_rules! with_app {
    (($app_ident:ident) => $code:block) => {{
        use crate::app::APP;

        APP.with_borrow(move |app_option| {
            let Some($app_ident) = app_option else {
                return Err(mlua::Error::RuntimeError("App is none".into()));
            };

            $code
        })
    }};
}
