use mlua::prelude::*;
use tao::window::WindowId;

pub struct LuaWindowId(pub WindowId);

impl LuaUserData for LuaWindowId {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, that: LuaAnyUserData| {
            let that = that.borrow::<Self>()?;
            Ok(this.0 == that.0)
        })
    }
}
