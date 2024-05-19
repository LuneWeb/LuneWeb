use crate::libraries::LuneWebLibraries;
use lune_std::context::GlobalsContextBuilder;
use mlua::prelude::*;
use mlua_luau_scheduler::Scheduler;
use std::{fs, path::PathBuf};

fn inject_globals(lua: &Lua) -> Result<(), LuaError> {
    let mut builder = GlobalsContextBuilder::new();

    builder.with_alias("luneweb", |modules| {
        for lib in LuneWebLibraries::ALL {
            modules.insert(lib.name(), lib.module_creator());
        }

        Ok(())
    })?;

    lune_std::inject_globals(lua, builder)?;

    Ok(())
}

pub async fn inject_lua<'lua>(lua: &'lua Lua, path: &'lua PathBuf) -> LuaResult<Scheduler<'lua>> {
    let sched = Scheduler::new(lua);
    let chunk = fs::read_to_string(&path)?;

    inject_globals(&lua)?;

    let main = lua.load(chunk).set_name(path.to_string_lossy().to_string());
    sched.push_thread_back(main, ())?;

    Ok(sched)
}
