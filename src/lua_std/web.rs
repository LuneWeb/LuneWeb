use crate::{
    lua_bindings::{tao::LuaWindow, wry::LuaWebView},
    utils::table_builder::TableBuilder,
    LuaAppProxyMethods,
};
use mlua::{ExternalResult, IntoLua};
use std::sync::Arc;
use tao::window::Window;

#[derive(Debug, Clone)]
pub struct WebViewProps {
    pub parent: Arc<Window>,
    pub scripts: Vec<String>,
}

impl mlua::FromLua for WebViewProps {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let t = value
            .as_table()
            .ok_or_else(|| mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "table".to_string(),
                message: None,
            })?;

        let parent = t.get::<mlua::AnyUserData>("parent")?;
        let parent = parent.borrow_scoped::<LuaWindow, Arc<Window>>(|window| window.0.clone())?;

        Ok(Self {
            parent,
            scripts: t.get("scripts").unwrap_or_default(),
        })
    }
}

pub(super) fn create(lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
    TableBuilder::new(lua)?
        .with_async_function(
            "createWindow",
            move |lua, title: Option<String>| async move {
                let window = lua.get_app_proxy().create_window(title).await;
                Ok(LuaWindow(window))
            },
        )?
        .with_function("createWebView", move |_, props: WebViewProps| {
            let mut builder = wry::WebViewBuilder::new();

            for script in props.scripts {
                builder = builder.with_initialization_script(&script);
            }

            let webview = builder.build(&props.parent).into_lua_err()?;

            Ok(LuaWebView(webview))
        })?
        .build_readonly()?
        .into_lua(lua)
}
