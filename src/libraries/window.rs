use super::event_loop::EVENT_LOOP;
use crate::classes::windowid::LuaWindowId;
use lune_utils::TableBuilder;
use mlua::prelude::*;
use std::rc::Rc;
use tao::window::{Window, WindowBuilder};
use wry::WebView;

pub struct LuaWindow {
    pub this: Window,

    // note: this keeps the webviews from being garbage collected
    pub webview: Option<Rc<WebView>>,

    closed: bool,
}

impl LuaUserData for LuaWindow {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(LuaWindowId(this.this.id())));
        fields.add_field_method_get("closed", |_, this| Ok(this.closed));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("close", |_, this, _: ()| {
            this.this.set_visible(false);
            this.closed = true;
            Ok(())
        });
    }
}

fn window_builder() -> WindowBuilder {
    #[cfg(target_os = "linux")]
    {
        use tao::platform::unix::WindowBuilderExtUnix;
        WindowBuilder::new().with_default_vbox(false)
    }

    #[cfg(not(target_os = "linux"))]
    WindowBuilder::new()
}

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("new", |_, _: ()| {
            let window_result = EVENT_LOOP.with(|event_loop| {
                let target = event_loop.borrow();

                window_builder().build(&target)
            });

            Ok(LuaWindow {
                this: window_result.unwrap(),
                webview: None,
                closed: false,
            })
        })?
        .build_readonly()
}
