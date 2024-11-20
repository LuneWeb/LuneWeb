use mlua::{UserDataFields, UserDataMethods};
use std::sync::Arc;

pub struct LuaWindow(pub Arc<tao::window::Window>);

impl mlua::UserData for LuaWindow {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |lua, this| lua.create_any_userdata(this.0.id()));

        fields.add_field_method_get("title", |_, this| Ok(this.0.title()));
        fields.add_field_method_set("title", |_, this, title: String| {
            this.0.set_title(&title);
            Ok(())
        });
        fields.add_field_method_get("visible", |_, this| Ok(this.0.is_visible()));
        fields.add_field_method_set("visible", |_, this, visible: bool| {
            this.0.set_visible(visible);
            Ok(())
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::Eq, |_, this, other: mlua::AnyUserData| {
            let other = other.take::<Self>()?;

            Ok(this.0.id() == other.0.id())
        });
    }
}

pub struct LuaWindowId(pub tao::window::WindowId);

impl mlua::UserData for LuaWindowId {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, _: ()| {
            Ok(format!("{:?}", this.0))
        });

        methods.add_meta_method(mlua::MetaMethod::Eq, |_, this, other: mlua::AnyUserData| {
            let other = other.take::<Self>()?;

            Ok(this.0 == other.0)
        });
    }
}
