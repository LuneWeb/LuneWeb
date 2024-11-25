use mlua::UserDataMethods;

#[derive(Clone)]
pub struct LuaWebView(pub String);

impl mlua::UserData for LuaWebView {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::Eq, |_, this, other: mlua::AnyUserData| {
            let other = other.take::<Self>()?;

            Ok(this.0 == other.0)
        });
    }
}
