use mlua::prelude::*;
use std::path::PathBuf;

pub async fn lua_require(lua: Lua, path: PathBuf) -> LuaResult<LuaMultiValue> {
    Ok(Default::default())
}
