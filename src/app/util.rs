macro_rules! patched_lua {
    ($lune_ctx:expr) => {{
        use mlua::Lua;
        use std::rc::Rc;

        let lua = Lua::new();
        lune_std::inject_globals(&lua, $lune_ctx)?;
        lua.sandbox(true)?;

        // sandboxing makes all the inserted globals read-only, so we should insert _G after sandboxing
        lune_std::LuneStandardGlobal::GTable.create(&lua, $lune_ctx)?;

        let lua_rc = Rc::new(lua);
        lua_rc.set_app_data(Rc::downgrade(&lua_rc));
        lua_rc
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
