use scheduler::Scheduler;

pub mod app;
pub mod lua_std;
mod scheduler;
pub mod utils;

pub const ALWAYS_SINGLE_THREAD: bool = false;
pub const WINDOW_DEFAULT_TITLE: &str = "LuauApp";

main!(|sched, proxy, lua| -> mlua::Result<()> {
    let thread = lua.create_thread(
        lua.load(smol::fs::read_to_string("app.luau").await?)
            .set_name("app.luau")
            .into_function()?,
    )?;

    lua.globals().set(
        "task",
        lua_std::StandardLibrary::Task.into_lua(&lua, &proxy)?,
    )?;

    lua.globals()
        .set("web", lua_std::StandardLibrary::Web.into_lua(&lua, &proxy)?)?;

    proxy.spawn_lua_thread(thread, None);

    Ok(())
});
