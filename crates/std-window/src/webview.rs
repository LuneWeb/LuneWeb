use mlua::ExternalResult;
use tao::window::WindowId;

use crate::inner_window;

pub struct LuaWebview {
    pub(crate) id: WindowId,
}

impl mlua::UserData for LuaWebview {
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

    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("url", |lua, this| {
            inner_window!(let window << lua, this.id);

            let Some(webview) = &window.webview else {
                return Err(mlua::Error::RuntimeError(
                    "WebView is missing from Window".into(),
                ));
            };

            webview.inner.url().into_lua_err()
        });
        fields.add_field_method_set("url", |lua, this, url: String| {
            inner_window!(let window << lua, this.id);

            let Some(webview) = &window.webview else {
                return Err(mlua::Error::RuntimeError(
                    "WebView is missing from Window".into(),
                ));
            };

            webview.inner.load_url(&url).into_lua_err()
        });
    }
}
