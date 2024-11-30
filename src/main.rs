use scheduler::Scheduler;
use std::path::PathBuf;

pub mod app;
pub mod lua_bindings;
pub mod lua_require;
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

fn main() -> mlua::Result<()> {
    let sched = Scheduler::new();
    let lua = mlua::Lua::new();

    lua.globals()
        .set("task", lua_std::StandardLibrary::Task.into_lua(&lua)?)?;

    lua.globals()
        .set("web", lua_std::StandardLibrary::Web.into_lua(&lua)?)?;

    lua.globals().set(
        "require",
        lua.create_async_function(lua_require::lua_require)?,
    )?;

    // keep lua alive until the end of the scope
    let _lua = lua.clone();

    scheduler::thread::initialize_threads(sched.clone(), |proxy| {
        if let Err(err) = smol::block_on::<mlua::Result<()>>(async move {
            let script_path = std::env::args().nth(1).unwrap_or("init.luau".to_string());

            lua.set_app_data(proxy.clone());

            sched
                .executor
                .spawn(lua_require::utils::load_script(
                    lua.clone(),
                    PathBuf::from(script_path),
                ))
                .detach();

            Ok(())
        }) {
            eprintln!("{err}");
            std::process::exit(1);
        };
    });

    Ok(())
}
