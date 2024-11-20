use scheduler::Scheduler;

pub mod app;
pub mod lua_bindings;
pub mod lua_std;
mod scheduler;
pub mod utils;

pub const ALWAYS_SINGLE_THREAD: bool = false;
pub const WINDOW_DEFAULT_TITLE: &str = "LuauApp";

pub trait LuaAppProxyMethods {
    fn get_app_proxy(&self) -> app::AppProxy;
}

impl LuaAppProxyMethods for mlua::Lua {
    fn get_app_proxy(&self) -> app::AppProxy {
        self.app_data_ref::<app::AppProxy>()
            .expect("AppProxy is not found in app data container")
            .clone()
    }
}

main!(|sched, proxy, lua| -> mlua::Result<()> {
    let thread = lua.create_thread(
        lua.load(smol::fs::read_to_string("app.luau").await?)
            .set_name("app.luau")
            .into_function()?,
    )?;

    lua.set_app_data(proxy.clone());

    lua.globals()
        .set("task", lua_std::StandardLibrary::Task.into_lua(&lua)?)?;

    lua.globals()
        .set("web", lua_std::StandardLibrary::Web.into_lua(&lua)?)?;

    proxy.spawn_lua_thread(thread, None);

    Ok(())
});
