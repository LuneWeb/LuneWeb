use crate::{app::AppProxy, utils::table_builder::TableBuilder};
use mlua::IntoLua;

pub(super) fn create(lua: &mlua::Lua, proxy: &AppProxy) -> mlua::Result<mlua::Value> {
    let create_window_proxy = proxy.clone();

    TableBuilder::new(lua)?
        .with_async_function("createWindow", move |lua, title: Option<String>| {
            let create_window_proxy = create_window_proxy.clone();

            async move {
                let window = create_window_proxy.create_window(title).await;
                lua.create_any_userdata(window)
            }
        })?
        .build_readonly()?
        .into_lua(lua)
}
