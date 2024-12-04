use crate::utils::path::{append_extension, clean_path, strip_alias};
use mlua::prelude::*;
use std::path::PathBuf;

pub mod utils;

pub async fn lua_require(lua: Lua, path: PathBuf) -> LuaResult<LuaMultiValue> {
    if let Some((alias, path)) = strip_alias(path.clone())? {
        Err(mlua::Error::runtime(
            "Aliases are not supported in requires yet",
        ))

        // TODO: find the final path by searching .luaurc files
        // and pass it to load_module
    } else {
        let explicit_prefix = path.components().next().is_some_and(|x| {
            matches!(
                x,
                std::path::Component::CurDir | std::path::Component::ParentDir
            )
        });

        if !explicit_prefix {
            return Err(mlua::Error::runtime(
                r#"must have either "@", "./", or "../" prefix"#,
            ));
        }

        let directory = PathBuf::from(
            lua.inspect_stack(2)
                .and_then(|x| x.source().source.map(|x| x.to_string()))
                .expect("Failed to find script path from stack"),
        )
        .parent()
        .map_or_else(|| PathBuf::new(), |x| x.to_path_buf());

        let relative_path = directory.join(append_extension(clean_path(path), "luau"));

        utils::load_module(lua, relative_path).await
    }
}
