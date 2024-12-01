use super::tao::LuaWindow;
use mlua::{ExternalResult, LuaSerdeExt, UserDataMethods};
use std::sync::Arc;
use tao::window::Window;

pub struct LuaWebViewBuilder(pub wry::WebViewBuilder<'static>);

unsafe impl Send for LuaWebViewBuilder {}

impl mlua::UserData for LuaWebViewBuilder {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function(
            "withInitScript",
            |_, (this_userdata, script): (mlua::AnyUserData, String)| {
                let mut this = this_userdata.take::<Self>()?;

                this.0 = this.0.with_initialization_script(&script);

                Ok(this)
            },
        );

        methods.add_function(
            "build",
            |_, (this_userdata, parent): (mlua::AnyUserData, mlua::AnyUserData)| {
                let this = this_userdata.take::<Self>()?;
                let parent =
                    parent.borrow_scoped::<LuaWindow, Arc<Window>>(|window| window.0.clone())?;

                Ok(LuaWebView(this.0.build(&parent).into_lua_err()?))
            },
        );
    }
}

pub struct LuaWebView(pub wry::WebView);

unsafe impl Send for LuaWebView {}

impl mlua::UserData for LuaWebView {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.0.id().to_string()));

        fields.add_field_method_get("url", |_, this| this.0.url().into_lua_err());
        fields.add_field_method_set("url", |_, this, url: String| {
            this.0.load_url(&url).into_lua_err()
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::Eq, |_, this, other: mlua::AnyUserData| {
            let other = other.take::<Self>()?;

            Ok(this.0.id() == other.0.id())
        });

        methods.add_method("loadHtml", |_, this, html: String| {
            this.0.load_html(&html).into_lua_err()
        });

        methods.add_method("getCookies", |_, this, _: ()| {
            this.0
                .cookies()
                .map(|x| x.iter().map(|x| x.to_string()).collect::<Vec<String>>())
                .into_lua_err()
        });

        methods.add_method("openDebugger", |_, this, _: ()| {
            this.0.open_devtools();

            Ok(())
        });

        methods.add_async_method("evaluate", |lua, this, script: String| async move {
            let (sender, receiver) = flume::bounded(1);

            this.0
                .evaluate_script_with_callback(&script, move |result| {
                    sender
                        .send(serde_json::from_str::<serde_json::Value>(&result))
                        .expect("Failed to send javascript result");
                })
                .into_lua_err()?;

            let json = receiver
                .recv_async()
                .await
                .expect("Failed to receive javascript result")
                .into_lua_err()?;

            if matches!(json, serde_json::Value::Null) {
                Ok(mlua::Nil)
            } else {
                lua.to_value(&json)
            }
        });
    }
}
