use mlua::prelude::*;
use std::sync::Arc;

pub fn inject_globals(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();
    let task = lua.create_table()?;

    task.set(
        "spawn",
        lua.create_function(|lua, (f, args): (mlua::Function, mlua::MultiValue)| {
            let executor = lua.app_data_ref::<Arc<smol::Executor>>().unwrap();

            executor.spawn(f.call_async::<()>(args)).detach();

            Ok(())
        })?,
    )?;

    globals.set("task", task)?;

    Ok(())
}
