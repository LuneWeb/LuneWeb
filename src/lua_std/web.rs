use crate::{
    lua_bindings::{tao::LuaWindow, wry::LuaWebView},
    scheduler::thread::LuaThreadMethods,
    utils::table_builder::TableBuilder,
    LuaAppProxyMethods,
};
use mlua::{ExternalResult, IntoLua};
use std::{collections::HashMap, sync::Arc};
use tao::window::Window;
use wry::http::Response;

#[derive(Debug, Clone)]
pub struct WebViewProps {
    pub parent: Arc<Window>,
    pub scripts: Vec<String>,
    pub servers: HashMap<String, mlua::Function>,
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
            servers: t.get("servers").unwrap_or_default(),
        })
    }
}

pub(super) fn create(lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
    TableBuilder::new(lua)?
        .with_async_function("createWindow", |lua, title: Option<String>| async move {
            let window = lua.get_app_proxy().create_window(title).await;
            Ok(LuaWindow(window))
        })?
        .with_async_function("createWebView", |lua, props: WebViewProps| async move {
            let mut builder = wry::WebViewBuilder::new();

            for script in props.scripts {
                builder = builder.with_initialization_script(&script);
            }

            for server in props.servers {
                let lua_inner = lua.clone();

                builder =
                    builder.with_asynchronous_custom_protocol(server.0, move |_, req, res| {
                        let thread = lua_inner
                            .create_thread(server.1.clone())
                            .expect("Failed to turn callback into thread");

                        let proxy = lua_inner.get_app_proxy();

                        let lua = lua_inner.clone();
                        lua_inner
                            .spawn(async move {
                                proxy.spawn_lua_thread(
                                    thread.clone(),
                                    Some(mlua::MultiValue::from_vec(vec![String::from_utf8(
                                        req.into_body(),
                                    )
                                    .unwrap()
                                    .into_lua(&lua)
                                    .unwrap()])),
                                );
                                res.respond::<Vec<u8>>(Response::new(
                                    proxy
                                        .await_lua_thread(thread)
                                        .await
                                        .expect("Server callback failed to give results")
                                        .get(0)
                                        .unwrap()
                                        .as_string_lossy()
                                        .unwrap()
                                        .into(),
                                ));
                            })
                            .detach();
                    });
            }

            let webview = builder.build(&props.parent).into_lua_err()?;

            Ok(LuaWebView(webview))
        })?
        .build_readonly()?
        .into_lua(lua)
}
