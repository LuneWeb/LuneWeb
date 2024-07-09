/// Get weak rc reference to lua and upgrade it
macro_rules! inner_lua {
    ($lua:expr) => {{
        use std::rc::Weak;

        $lua.app_data_ref::<Weak<mlua::Lua>>()
            .expect("Missing weak lua reference")
            .upgrade()
            .expect("Failed to upgrade weak lua reference")
    }};
}

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
