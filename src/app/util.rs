macro_rules! patched_lua {
    () => {{
        use mlua::Lua;
        use std::rc::Rc;

        let lua = Lua::new();
        let lua_rc = Rc::new(lua);
        lua_rc.set_app_data(Rc::downgrade(&lua_rc));
        lua_rc
    }};
}

#[macro_export]
macro_rules! include_luau {
    ($luau_dir:expr, $path:literal) => {{
        use lune_std::context::GlobalsContextBuilder;
        use std::env::current_dir;

        let mut ctx = GlobalsContextBuilder::new();
        let cwd = current_dir().expect("Failed to get current working directory");

        for file in $luau_dir.files() {
            let Some(ext) = file.path().extension() else {
                continue;
            };

            if ext == "luau" {
                ctx.with_script(cwd.join($path).join(file.path()), file.contents().into());
            }
        }

        ctx
    }};
}

/// Create a window builder that has cross-platform support
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
