use std::rc::Weak;

use mlua::ExternalResult;
use mlua_luau_scheduler::{IntoLuaThread, LuaSchedulerExt, LuaSpawnExt};
use tokio::sync::watch::channel;

use crate::{APP, ONLOAD_TX};

pub const JS_IMPL: &str = include_str!(".js");

const LUA_SERDE_CONFIG: lune_std_serde::EncodeDecodeConfig = lune_std_serde::EncodeDecodeConfig {
    format: lune_std_serde::EncodeDecodeFormat::Json,
    pretty: false,
};

pub fn create(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    lune_utils::TableBuilder::new(lua)?
        .with_function("share", message_share)?
        .with_async_function("send", message_send)?
        .with_function("onLoad", |lua, function: mlua::Function| {
            let mut rx = ONLOAD_TX.with_borrow(|tx| tx.subscribe());
            let reg_key_function = lua.create_registry_value(function)?;

            let inner_lua = lua
                .app_data_ref::<Weak<mlua::Lua>>()
                .expect("Missing weak lua reference")
                .upgrade()
                .unwrap();

            lua.spawn_local(async move {
                loop {
                    let _ = rx.changed().await;
                    let function: mlua::Function =
                        inner_lua.registry_value(&reg_key_function).unwrap();

                    let thread = function.into_lua_thread(&inner_lua).unwrap();
                    inner_lua.push_thread_back(thread, ()).unwrap();
                }
            });

            Ok(())
        })?
        .build_readonly()
}

fn message_share(lua: &mlua::Lua, message: mlua::Value) -> Result<(), mlua::Error> {
    let json = lune_std_serde::encode(message, lua, LUA_SERDE_CONFIG)?;

    APP.with_borrow(|app| {
        if let Some(webview) = &app.webview {
            let string_json = json.to_string_lossy();
            let script = format!("window.luneweb.shareMessage({string_json})");

            webview.evaluate_script(&script).into_lua_err()
        } else {
            Err(mlua::Error::RuntimeError("WebView not available".into()))
        }
    })
}

async fn message_send<'lua>(
    lua: &'lua mlua::Lua,
    (channel_name, message): (mlua::Value<'lua>, mlua::Value<'lua>),
) -> Result<mlua::Value<'lua>, mlua::Error> {
    let json = lune_std_serde::encode(message, lua, LUA_SERDE_CONFIG)?;

    let (tx, mut rx) = channel::<String>("null".into());

    APP.with_borrow(move |app| {
        if let Some(webview) = &app.webview {
            let string_json = json.to_string_lossy();
            let string_channel_name = channel_name.to_string()?;
            let script =
                format!(r#"window.luneweb.sendMessage("{string_channel_name}", {string_json})"#);

            webview
                .evaluate_script_with_callback(&script, move |received| {
                    tx.send(received).unwrap();
                })
                .into_lua_err()?;

            Ok(())
        } else {
            Err(mlua::Error::RuntimeError("WebView not available".into()))
        }
    })?;

    let _ = rx.changed().await;
    let received = rx.borrow_and_update();

    lune_std_serde::decode(received.to_string(), lua, LUA_SERDE_CONFIG)
}
