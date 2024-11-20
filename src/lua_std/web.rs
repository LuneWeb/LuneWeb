use crate::{lua_bindings::tao::LuaWindow, utils::table_builder::TableBuilder, LuaAppProxyMethods};
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
        .build_readonly()?
        .into_lua(lua)
}
