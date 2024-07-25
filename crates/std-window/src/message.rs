use mlua::ExternalResult;
use mlua_luau_scheduler::LuaSchedulerExt;
use tao::window::WindowId;
use tokio_stream::{wrappers::WatchStream, StreamExt};

use crate::{
    inner_window,
    serde::{json_to_lua, lua_to_json},
};

pub struct LuaMessage {
    pub(crate) id: WindowId,
}

impl mlua::UserData for LuaMessage {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("listen", |lua, this, _: ()| {
            inner_window!(let window << lua, this.id);

            let Some(webview) = &window.webview else {
                return Err(mlua::Error::RuntimeError(
                    "WebView is missing from Window".into(),
                ));
            };

            let rx = webview.messages.clone();

            #[allow(unreachable_code)]
            Ok(mlua::Function::wrap_async(
                move |lua, callback: mlua::Function| {
                    let rx_inner = rx.clone();

                    async move {
                        let rx = rx_inner.clone();
                        let mut stream = WatchStream::from_changes(rx);

                        loop {
                            if let Some(value) = stream.next().await {
                                lua.push_thread_front(callback.clone(), json_to_lua(value, lua))?;
                            }
                        }

                        Ok(())
                    }
                },
            ))
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
