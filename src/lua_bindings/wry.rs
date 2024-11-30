use mlua::UserDataMethods;

pub struct LuaWebView(pub wry::WebView);

unsafe impl Send for LuaWebView {}

impl mlua::UserData for LuaWebView {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::Eq, |_, this, other: mlua::AnyUserData| {
            let other = other.take::<Self>()?;

            Ok(this.0.id() == other.0.id())
        });
    }
}
