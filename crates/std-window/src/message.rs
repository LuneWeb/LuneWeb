use mlua::ExternalResult;
use tao::window::WindowId;

use crate::inner_window;

pub struct LuaMessage {
    pub(crate) id: WindowId,
}

impl mlua::UserData for LuaMessage {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("eval", |lua, this, src: String| {
            inner_window!(let window << lua, this.id);

            let Some(webview) = &window.webview else {
                return Err(mlua::Error::RuntimeError(
                    "WebView is missing from Window".into(),
                ));
            };

            webview.inner.evaluate_script(&src).into_lua_err()?;

            Ok(())
        });
    }
}
