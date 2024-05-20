use super::{IPCChannel, IPCMessage, LuaWebView};
use crate::{
    classes::{
        connection::LuaConnection,
        http::{request::LuaRequest, response::LuaResponse},
    },
    libraries::{webview::INIT_SCRIPT, window::LuaWindow},
};
use http::{Request, Response};
use mlua::prelude::*;
use mlua_luau_scheduler::LuaSpawnExt;
use serde::Deserialize;
use std::{
    borrow::Cow,
    collections::HashMap,
    rc::{Rc, Weak},
    sync::Mutex,
    time::Duration,
};
use tao::window::Window;
use wry::WebViewBuilder;

#[derive(Deserialize, Debug)]
pub struct InternalIPCMessage {
    pub __internal: bool,
    pub action: String,
    pub data: serde_json::Value,
}

#[derive(Debug)]
pub struct CustomProtocolInfo {
    pub request_channel: tokio::sync::watch::Sender<Request<Vec<u8>>>,
    pub response_channel: tokio::sync::watch::Sender<Response<Cow<'static, [u8]>>>,
}

#[derive(Default)]
pub(super) struct LuaWebViewBuilder {
    pub url: Option<String>,
    pub initialization_script: Option<String>,
    pub custom_protocols: HashMap<String, CustomProtocolInfo>,
}

impl LuaWebViewBuilder {
    fn into_builder<'a>(&self, target: &'a Window) -> WebViewBuilder<'a> {
        #[cfg(not(target_os = "linux"))]
        {
            WebViewBuilder::new(target)
        }

        #[cfg(target_os = "linux")]
        {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            WebViewBuilder::new_gtk(target.gtk_window())
        }
    }
}

impl LuaUserData for LuaWebViewBuilder {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("with_url", |_, this, url: String| {
            this.url = Some(url);
            Ok(())
        });

        methods.add_method_mut("with_initialization_script", |_, this, url: String| {
            this.initialization_script = Some(url);
            Ok(())
        });

        methods.add_method_mut(
            "with_custom_protocol",
            |lua, this, (protocol, handler): (String, LuaFunction)| {
                use tokio::sync::watch::Sender;

                for char in protocol.chars() {
                    if char.is_uppercase() {
                        return Err(LuaError::RuntimeError(format!("Custom protocol name '{protocol}' is not valid, only the first character is allowed to be uppercased")))
                    }
                }

                let info = CustomProtocolInfo {
                    request_channel: Sender::new(Request::default()),
                    response_channel: Sender::new(Response::default()),
                };

                let mut inner_rq = info.request_channel.subscribe();
                let inner_res = info.response_channel.clone();

                this.custom_protocols.insert(protocol, info);

                let key_handler = lua.create_registry_value(handler)?;

                let inner_lua = lua
                    .app_data_ref::<Weak<Lua>>()
                    .expect("Missing weak lua ref")
                    .upgrade()
                    .expect("Lua was dropped unexpectedly");

                let connection = LuaConnection::new();
                let mut shutdown_tx = connection.shutdown_tx.subscribe();

                lua.spawn_local(async move {
                    loop {
                        tokio::select! {
                            Ok(_) = shutdown_tx.changed() => break,
                            _ = inner_rq.changed() => {}
                        }

                        let inner_handler = inner_lua.registry_value::<LuaFunction>(&key_handler);

                        if let Ok(handler) = inner_handler {
                            let request = inner_rq.borrow_and_update();
                            let lua_req = LuaRequest {
                                body: request.body().to_vec(),
                                head: request.clone().into_parts().0,
                            };

                            let lua_res = handler
                                .call_async::<_, LuaResponse>(lua_req.into_lua_table(&inner_lua))
                                .await
                                .expect("Expected a response for custom protocol from lua");

                            let _ = inner_res.send(
                                lua_res
                                    .into_response::<Cow<'static, [u8]>>()
                                    .expect("Lua response is not valid"),
                            );
                        }
                    }
                });

                Ok(connection)
            },
        );

        methods.add_method("build", |lua, this, target: LuaAnyUserData| {
            let mut target = target.borrow_mut::<LuaWindow>()?;
            let url = this.url.clone();

            let ipc_channel = IPCChannel::new(IPCMessage { channel: "_".into(), data: serde_json::Value::Null });
            let inner_ipc_channel = ipc_channel.clone();

            //  let _ = channel_pool.send(IPCMessage { channel: message.channel, data: message.data });
            let channel_pool: Rc<Mutex<Vec<IPCMessage>>> = Rc::new(Mutex::new(vec![]));
            let inner_channel_pool = Rc::clone(&channel_pool);

            lua.spawn_local(async move {
                loop {
                    if inner_ipc_channel.receiver_count() == 0 {
                        break;
                    }

                    if let Ok(mut channel_pool) = inner_channel_pool.try_lock() {
                        if !channel_pool.is_empty() {
                            let _ = inner_ipc_channel.send(channel_pool.pop().unwrap());
                        }
                    }

                    tokio::time::sleep(Duration::from_millis(16)).await;
                }
            });

            let mut builder = this
                .into_builder(&target.this)
                .with_ipc_handler(move |data|  {
                    let body = data.body().as_str();

                    let internal_message: Result<InternalIPCMessage, serde_json::Error> =
                        serde_json::from_str(body);

                    let message: Result<IPCMessage, serde_json::Error> = serde_json::from_str(body);

                    if let Ok(message) = internal_message {
                        match message.action.as_str() {
                            "print" => {
                                let text: String =
                                    serde_json::from_value(message.data).expect("Failed to turn message.data into string for internal action 'print'");
                                println!("{text}");
                            }
                            _ => {
                                unimplemented!(
                                    "{} internal action is not implemented",
                                    message.action,
                                )
                            }
                        }
                    } else if let Ok(message) = message {
                        channel_pool.lock().unwrap().push(message);
                    }
                })
                .with_initialization_script(&{
                    let mut src = INIT_SCRIPT.clone();

                    if let Some(addon) = &this.initialization_script {
                        src += &("\n".to_owned() + addon);
                    }

                    format!("window.onload = () => {{ {} }}", src)
                })
                .with_url(url.unwrap_or("about:blank".into()));

            for (protocol, info) in &this.custom_protocols {
                let inner_lua = lua
                    .app_data_ref::<Weak<Lua>>()
                    .expect("Missing weak lua ref")
                    .upgrade()
                    .expect("Lua was dropped unexpectedly");

                let rx_response = info.response_channel.subscribe();
                let tx_request = info.request_channel.clone();

                builder = builder.with_asynchronous_custom_protocol(protocol.to_string(), move |request, responder| {
                    if tx_request.receiver_count() == 0 {
                        // no listeners
                        responder.respond::<Cow<'static, [u8]>>(Response::default());
                        return;
                    }

                    let mut inner_rx_response = rx_response.clone();
                    let _ = tx_request.send(request);

                    inner_lua.spawn_local(async move {
                        let _ = inner_rx_response.changed().await;
                        let response = inner_rx_response.borrow_and_update();

                        responder.respond(response.to_owned());
                    });
                })
            }

            let webview = builder.build().into_lua_err()?;
            let webview_rc = Rc::new(webview);

            {
                let inner_webview_rc = Rc::clone(&webview_rc);
                target.webview = Some(inner_webview_rc);
            }

            Ok(LuaWebView { this: webview_rc, ipc_channel })
        });
    }
}
