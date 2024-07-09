use mlua::ExternalResult;
use mlua_luau_scheduler::{IntoLuaThread, LuaSchedulerExt, LuaSpawnExt};
use tokio::sync::watch;

use crate::{serde, App};

pub fn implement_lua_methods<'lua, M: mlua::UserDataMethods<'lua, App>>(methods: &mut M) {
    methods.add_method("onLoad", |lua, app, function: mlua::Function| {
        let mut rx = app.load.subscribe();
        let inner_lua = inner_lua!(lua);
        let reg_key_function = lua.create_registry_value(function)?;

        lua.spawn_local(async move {
            loop {
                let _ = rx.changed().await;

                let function: mlua::Function = inner_lua
                    .registry_value(&reg_key_function)
                    .expect("Failed to get onLoad callback from lua registry");

                let thread = function
                    .into_lua_thread(&inner_lua)
                    .expect("Failed to turn lua function into lua thread");

                inner_lua
                    .push_thread_back(thread, ())
                    .expect("Failed to push thread back");
            }
        });

        Ok(())
    });

    methods.add_method("shareMessage", |lua, app, message: mlua::Value| {
        let json = serde::lua_to_json(message, lua)?;
        let script = format!("window.luneweb.shareMessage({0})", json.to_string_lossy());
        app.webview.evaluate_script(&script).into_lua_err()
    });

    methods.add_async_method(
        "sendMessage",
        |lua, app, (channel, message): (String, mlua::Value)| async move {
            let (tx, mut rx) = watch::channel::<String>("null".into());
            let json = serde::lua_to_json(message, lua)?;
            let script = format!(
                r#"window.luneweb.sendMessage("{channel}", {0})"#,
                json.to_string_lossy()
            );

            app.webview
                .evaluate_script_with_callback(&script, move |received| {
                    tx.send(received)
                        .expect("Failed to send returned value from webview");
                })
                .into_lua_err()?;

            let _ = rx.changed().await;
            let received = rx.borrow_and_update();

            serde::json_to_lua(received.to_string(), lua)
        },
    );
}
