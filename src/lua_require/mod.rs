use crate::{
    utils::path::{append_extension, clean_path, strip_alias},
    LuaAppProxyMethods,
};
use mlua::prelude::*;
use std::path::PathBuf;

async fn load_module(lua: mlua::Lua, path: PathBuf) -> LuaResult<LuaMultiValue> {
    let contents = smol::fs::read_to_string(&path)
        .await
        .map_err(|x| match x.kind() {
            std::io::ErrorKind::NotFound => mlua::Error::runtime(format!(
                "The system cannot find {} (os error 2)",
                path.to_string_lossy()
            )),
            _ => x.into_lua_err(),
        })?;

    let chunk = lua.load(contents).set_name(path.to_string_lossy());
    let thread = lua.create_thread(chunk.into_function()?)?;
    let proxy = lua.get_app_proxy();

    proxy.spawn_lua_thread(thread.clone(), None);
    let result = proxy.await_lua_thread(thread).await;

    result
}

pub async fn lua_require(lua: Lua, path: PathBuf) -> LuaResult<LuaMultiValue> {
    let path = append_extension(clean_path(path), "luau");

    if let Some((alias, path)) = strip_alias(path.clone())? {
        Err(mlua::Error::runtime(
            "Aliases are not supported in requires yet",
        ))

        // TODO: find the final path by searching .luaurc files
        // and pass it to load_module
    } else {
        load_module(lua, path).await
    }
}
