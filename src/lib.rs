use std::{path::PathBuf, rc::Rc};

use lua::inject_lua;
use mlua::prelude::*;

mod classes;
mod libraries;
mod lua;

pub fn patch_lua(lua: &Rc<Lua>) {
    lua.set_app_data(Rc::downgrade(&lua));
}

pub async fn init<'lua>(
    lua: &'lua Lua,
    path: &'lua PathBuf,
) -> Result<mlua_luau_scheduler::Scheduler<'lua>, LuaError> {
    inject_lua(lua, path).await
}
