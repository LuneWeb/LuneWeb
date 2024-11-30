use mlua::{ExternalResult, UserDataMethods};

pub struct LuaWebView(pub wry::WebView);

unsafe impl Send for LuaWebView {}

impl mlua::UserData for LuaWebView {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.0.id().to_string()));
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::Eq, |_, this, other: mlua::AnyUserData| {
            let other = other.take::<Self>()?;

            Ok(this.0.id() == other.0.id())
        });

        methods.add_method("loadHtml", |_, this, html: String| {
            this.0.load_html(&html).into_lua_err()
        });

        methods.add_method("loadUrl", |_, this, url: String| {
            this.0.load_url(&url).into_lua_err()
        });
    }
}
