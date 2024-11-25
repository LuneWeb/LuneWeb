use mlua::{ExternalResult, UserDataFields, UserDataMethods};

pub struct LuaWebView {
    pub inner: wry::WebView,
    pub visible: bool,
}

impl mlua::UserData for LuaWebView {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.inner.id().to_string()));

        fields.add_field_method_get("visible", |_, this| Ok(this.visible));
        fields.add_field_method_set("visible", |_, this, visible: bool| {
            this.inner.set_visible(visible).into_lua_err()?;
            this.visible = visible;
            Ok(())
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::Eq, |_, this, other: mlua::AnyUserData| {
            let other = other.take::<Self>()?;

            Ok(this.inner.id() == other.inner.id())
        });
    }
}
