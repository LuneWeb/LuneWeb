use crate::utils::path::{clean_path, strip_alias};
use mlua::prelude::*;
use std::path::PathBuf;

pub async fn lua_require(lua: Lua, path: PathBuf) -> LuaResult<LuaMultiValue> {
    let path = clean_path(path);

    if let Some((alias, path)) = strip_alias(path)? {
        Err(mlua::Error::runtime(
            "Aliases are not supported in requires yet",
        ))
    } else {
        Ok(Default::default())
    }
}
