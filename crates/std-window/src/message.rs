use std::{
    rc::{Rc, Weak},
    time::Duration,
};

use mlua::ExternalResult;
use mlua_luau_scheduler::{LuaSchedulerExt, LuaSpawnExt};
use tao::window::WindowId;
use tokio::sync::watch;
use tokio_stream::{wrappers::WatchStream, StreamExt};

use crate::{
    inner_window,
    serde::{json_to_lua, lua_to_json},
};

/// The speed of the loop that reads the messages sent from WebView
const POOL_INTERVAL: Duration = Duration::from_millis(16);

pub struct LuaMessage {
    pub(crate) id: WindowId,
    pub(crate) tx: Rc<watch::Sender<String>>,
}

impl LuaMessage {
    pub fn watch_pool(&self, lua: &mlua::Lua) -> Result<(), mlua::Error> {
        inner_window!(let window << lua, self.id);

        let Some(webview) = &window.webview else {
            return Err(mlua::Error::RuntimeError(
                "WebView is missing from Window".into(),
            ));
        };

        let pool: Rc<std::sync::Mutex<Vec<String>>> = Rc::clone(&webview.message_pool);
        let tx = Rc::clone(&self.tx);

        lua.spawn_local(async move {
            let mut interval = tokio::time::interval(POOL_INTERVAL);

            loop {
                if let Ok(mut pool) = pool.try_lock() {
                    if let Some(message) = pool.pop() {
                        let _ = tx.send(message);
                    }
                }

                interval.tick().await;
            }
        });

        Ok(())
    }
}

impl mlua::UserData for LuaMessage {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        #[allow(unreachable_code)]
        methods.add_async_method("listen", |lua, this, callback: mlua::Function| async move {
            let callback_reg_key = lua.create_registry_value(callback)?;
            let (dc_tx, mut dc_rx) = watch::channel(false);
            let mut stream = WatchStream::from_changes(this.tx.subscribe());

            if Rc::strong_count(&this.tx) < 2 {
                this.watch_pool(lua)?;
            }

            let inner_lua = lua
                .app_data_ref::<Weak<mlua::Lua>>()
                .expect("Missing weak lua reference")
                .upgrade()
                .expect("Failed to upgrade weak pointer to lua");

            lua.spawn_local(async move {
                let callback: mlua::Function = inner_lua
                    .registry_value(&callback_reg_key)
                    .expect("Failed to get callback function from registry");

                loop {
                    tokio::time::sleep(Duration::from_millis(16)).await;

                    tokio::select! {
                        result = dc_rx.changed() => {
                            if result.is_ok() {
                                break
                            }
                        },
                        Some(message) = stream.next() => {
                            inner_lua
                            .push_thread_front(callback.clone(), json_to_lua(message, &inner_lua))
                            .expect("Failed to call callback function");
                        }
                    }
                }
            });

            Ok(mlua::Function::wrap(move |_, _: ()| {
                let _ = dc_tx.send(true);

                Ok(())
            }))
        });

        methods.add_method(
            "send",
            |lua, this, (channel, message): (String, mlua::Value)| {
                inner_window!(let window << lua, this.id);

                let Some(webview) = &window.webview else {
                    return Err(mlua::Error::RuntimeError(
                        "WebView is missing from Window".into(),
                    ));
                };

                let js_message = lua_to_json(message, lua)?.to_string_lossy().to_string();

                webview.call_js_channel(channel, js_message).into_lua_err()
            },
        );
    }
}
