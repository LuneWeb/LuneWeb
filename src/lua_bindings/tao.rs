use mlua::{UserDataFields, UserDataMethods};
use std::sync::Arc;

pub fn register(lua: &mlua::Lua) -> mlua::Result<()> {
    lua.register_userdata_type::<tao::window::WindowId>(|registry| {
        registry.add_meta_method(mlua::MetaMethod::ToString, |_, this, _: ()| {
            Ok(format!("{this:?}"))
        });

        registry.add_meta_method(mlua::MetaMethod::Eq, |_, this, other: mlua::AnyUserData| {
            let other = other.take::<tao::window::WindowId>()?;

            Ok(*this == other)
        });
    })?;

    lua.register_userdata_type::<Arc<tao::window::Window>>(|registry| {
        registry.add_field_method_get("id", |lua, this| lua.create_any_userdata(this.id()));

        registry.add_field_method_get("title", |_, this| Ok(this.title()));
        registry.add_field_method_set("title", |_, this, title: String| {
            this.set_title(&title);
            Ok(())
        });
        registry.add_field_method_get("visible", |_, this| Ok(this.is_visible()));
        registry.add_field_method_set("visible", |_, this, visible: bool| {
            this.set_visible(visible);
            Ok(())
        });

        registry.add_meta_method(mlua::MetaMethod::Eq, |_, this, other: mlua::AnyUserData| {
            let other = other.take::<Arc<tao::window::Window>>()?;

            Ok(this.id() == other.id())
        });
    })?;

    Ok(())
}
