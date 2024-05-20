use super::{IPCChannel, IPCMessage, LuaWebView};
use crate::libraries::{webview::INIT_SCRIPT, window::LuaWindow};
use mlua::prelude::*;
use mlua_luau_scheduler::LuaSpawnExt;
use serde::Deserialize;
use std::{rc::Rc, sync::Mutex, time::Duration};
use tao::window::Window;
use wry::WebViewBuilder;

#[derive(Deserialize, Debug)]
pub struct InternalIPCMessage {
    pub __internal: bool,
    pub action: String,
    pub data: serde_json::Value,
}

#[derive(Default)]
pub(super) struct LuaWebViewBuilder {
    pub url: Option<String>,
    pub initialization_script: Option<String>,
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

            let builder = this
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
