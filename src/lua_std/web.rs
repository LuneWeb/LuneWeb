use crate::{
    lua_bindings::{tao::LuaWindow, wry::LuaWebView},
    utils::table_builder::TableBuilder,
    LuaAppProxyMethods,
};
use mlua::{ExternalResult, IntoLua};
use std::sync::Arc;
use tao::window::Window;

pub(super) fn create(lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
    TableBuilder::new(lua)?
        .with_async_function(
            "createWindow",
            move |lua, title: Option<String>| async move {
                let window = lua.get_app_proxy().create_window(title).await;
                Ok(LuaWindow(window))
            },
        )?
        .with_function("createWebView", move |_, window: mlua::AnyUserData| {
            let window =
                window.borrow_scoped::<LuaWindow, Arc<Window>>(|window| window.0.clone())?;
            let webview = wry::WebViewBuilder::new().build(&window).into_lua_err()?;

            Ok(LuaWebView(webview))
        })?
        .build_readonly()?
        .into_lua(lua)
}
