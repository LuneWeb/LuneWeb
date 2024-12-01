use crate::{
    lua_bindings::{tao::LuaWindow, wry::LuaWebViewBuilder},
    utils::table_builder::TableBuilder,
    LuaAppProxyMethods,
};
use mlua::IntoLua;

pub(super) fn create(lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
    TableBuilder::new(lua)?
        .with_async_function("createWindow", |lua, title: Option<String>| async move {
            let window = lua.get_app_proxy().create_window(title).await;
            Ok(LuaWindow(window))
        })?
        .with_function("createWebView", |_, _: ()| {
            Ok(LuaWebViewBuilder(wry::WebViewBuilder::new()))
        })?
        .build_readonly()?
        .into_lua(lua)
}
