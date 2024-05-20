use self::builder::LuaWebViewBuilder;
use crate::classes::connection::LuaConnection;
use lune_std_serde::{decode, EncodeDecodeConfig, EncodeDecodeFormat};
use lune_utils::TableBuilder;
use mlua::prelude::*;
use mlua_luau_scheduler::{LuaSchedulerExt, LuaSpawnExt};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::rc::{Rc, Weak};
use wry::WebView;

mod builder;

const LUA_SERIALIZE_OPTIONS: LuaSerializeOptions = LuaSerializeOptions::new()
    .set_array_metatable(false)
    .serialize_none_to_null(false)
    .serialize_unit_to_null(false);

const INIT_SCRIPT: Lazy<String> = Lazy::new(|| {
    let init_script = include_str!("init.js");
    init_script.into()
});

#[derive(Deserialize, Debug)]
pub struct IPCMessage {
    pub channel: String,
    pub data: serde_json::Value,
}

pub type IPCChannel = tokio::sync::watch::Sender<IPCMessage>;

struct LuaWebView {
    #[allow(dead_code)]
    pub this: Rc<WebView>,
    pub ipc_channel: IPCChannel,
}

impl LuaUserData for LuaWebView {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method(
            "evaluate_script_with_response",
            |lua, this, script: String| async move {
                let (tx, mut rx) = tokio::sync::watch::channel("null".to_string());
                let inner_tx = tx.clone();

                this.this
                    .evaluate_script_with_callback(script.as_str(), move |res| {
                        let _ = inner_tx.send(res.clone());
                    })
                    .expect("Failed to evaluate script");

                match rx.changed().await {
                    Ok(_) => {
                        let borrowed = rx.borrow_and_update();
                        let config = EncodeDecodeConfig::from(EncodeDecodeFormat::Json);
                        decode(borrowed.as_bytes(), lua, config)
                    }
                    Err(_) => Ok(LuaValue::Nil),
                }
            },
        );

        methods.add_async_method("evaluate_script", |_, this, script: String| async move {
            this.this.evaluate_script(script.as_str()).into_lua_err()
        });

        methods.add_method_mut(
            "subscribe",
            |lua, this, (channel, callback): (String, LuaFunction)| {
                let mut rx = this.ipc_channel.subscribe();

                let key_callback = lua
                    .create_registry_value(callback)
                    .expect("Failed to create registry value for callback");

                let inner_lua = lua
                    .app_data_ref::<Weak<Lua>>()
                    .expect("Missing weak lua ref")
                    .upgrade()
                    .expect("Lua was dropped unexpectedly");

                let connection = LuaConnection::new();
                let mut shutdown_rx = connection.shutdown_tx.subscribe();

                lua.spawn_local(async move {
                    loop {
                        tokio::select! {
                            Ok(_) = shutdown_rx.changed() => break,
                            _ = rx.changed() => {},
                        }

                        let inner_callback = inner_lua.registry_value::<LuaFunction>(&key_callback);

                        if let Ok(inner_callback) = inner_callback {
                            let message = rx.borrow_and_update();
                            let (requested_channel, requested_value) =
                                (&message.channel, &message.data);

                            if *requested_channel == channel {
                                let decoded = inner_lua
                                    .to_value_with(requested_value, LUA_SERIALIZE_OPTIONS)
                                    .expect("Failed to decode javascript value into lua value");

                                let thread =
                                    inner_lua.create_thread(inner_callback.clone()).unwrap();

                                inner_lua
                                    .as_ref()
                                    .push_thread_back(thread, decoded)
                                    .unwrap();
                            }
                        }
                    }
                });

                Ok(connection)
            },
        );
    }
}

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("new", |_, _: ()| {
            let lua_webview_builder = LuaWebViewBuilder::default();

            Ok(lua_webview_builder)
        })?
        .build_readonly()
}
