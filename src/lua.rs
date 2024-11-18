use mlua::prelude::*;

pub fn inject_globals(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();
    let task = lua.create_table()?;
    let app = lua.create_table()?;

    task.set(
        "spawn",
        lua.create_function(|lua, (f, args): (mlua::Function, mlua::MultiValue)| {
            let sched = lua.app_data_ref::<crate::scheduler::Scheduler>().unwrap();

            sched.executor.spawn(f.call_async::<()>(args)).detach();

            Ok(())
        })?,
    )?;

    app.set(
        "createWindow",
        lua.create_function(|lua, _: ()| {
            let proxy = lua.app_data_ref::<crate::app::AppProxy>().unwrap();
            let window = proxy.create_window();

            Ok(())
        })?,
    )?;

    globals.set("task", task)?;
    globals.set("app", app)?;

    Ok(())
}
