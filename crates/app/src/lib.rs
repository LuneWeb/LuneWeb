use std::{cell::RefCell, rc::Rc};

use mlua::{ExternalResult, Lua, MetaMethod};
use tao::{
    event_loop::{EventLoop, EventLoopBuilder},
    window::Window,
};
use tokio::sync::watch;
use wry::WebView;

#[macro_use]
pub mod macros;

pub mod config;
mod logic;
mod message;
pub mod serde;

pub struct App {
    pub event_loop: RefCell<EventLoop<()>>,
    pub window: Rc<Window>,
    pub webview: Rc<WebView>,
    pub load: watch::Sender<()>,
}

impl App {
    /// Insert `App` into lua's globals table
    pub fn into_global(self, lua: &Lua) -> Result<(), mlua::Error> {
        let globals = lua.globals();

        globals.set("app", self)?;

        Ok(())
    }

    pub fn new(config: config::AppConfig) -> Result<Self, mlua::Error> {
        let event_loop = EventLoopBuilder::new().build();

        let load = watch::Sender::new(());
        let inner_load = load.clone();

        let builder_window = window_builder!().with_title(config.window_title);
        let window = Rc::new(builder_window.build(&event_loop).into_lua_err()?);

        let builder_webview = webview_builder!(window)
            .with_initialization_script(&format!("{{ {0} }}", message::JS_IMPL))
            .with_url(config.url)
            .with_ipc_handler(move |_message| {
                if inner_load.receiver_count() > 0 {
                    inner_load
                        .send(())
                        .expect("Failed to send value to load channel");
                }
            });

        let webview = Rc::new(builder_webview.build().into_lua_err()?);
        let app = Self {
            event_loop: RefCell::new(event_loop),
            window,
            webview,
            load,
        };

        Ok(app)
    }
}

impl mlua::UserData for App {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function_mut(MetaMethod::Type, |_, _: ()| Ok("App"));

        message::methods::implement_lua_methods(methods);
        logic::methods::implement_lua_methods(methods);

        #[allow(unreachable_code)]
        methods.add_function("exit", |_, exit_code: i32| {
            Ok(std::process::exit(exit_code))
        });
    }
}
