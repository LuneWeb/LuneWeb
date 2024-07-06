use crate::LuneWebError;
use mlua::Lua;
use std::rc::Rc;

pub fn patched_lua(lune_ctx: &lune_std::context::GlobalsContext) -> Result<Rc<Lua>, LuneWebError> {
    let lua = Lua::new();
    lune_std::inject_globals(&lua, lune_ctx)?;

    // sandboxing makes all the inserted globals read-only
    // so we should insert _G again after sandboxing
    lua.sandbox(true)?;
    lune_std::LuneStandardGlobal::GTable.create(&lua, lune_ctx)?;

    let lua_rc = Rc::new(lua);
    lua_rc.set_app_data(Rc::downgrade(&lua_rc));
    Ok(lua_rc)
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
