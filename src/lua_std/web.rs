use crate::{
    lua_bindings::{tao::LuaWindow, wry::LuaWebView},
    utils::table_builder::TableBuilder,
    LuaAppProxyMethods,
};
use mlua::IntoLua;

pub(super) fn create(lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
    TableBuilder::new(lua)?
        .with_async_function(
            "createWindow",
            move |lua, title: Option<String>| async move {
                let window = lua.get_app_proxy().create_window(title).await;
                Ok(LuaWindow(window))
            },
        )?
        .with_async_function(
            "createWebView",
            move |lua, window: mlua::AnyUserData| async move {
                let window = window.take::<LuaWindow>()?;
                let webview = lua.get_app_proxy().create_webview(window.0).await;
                Ok(LuaWebView(webview))
            },
        )?
        .build_readonly()?
        .into_lua(lua)
}
