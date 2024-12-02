use scheduler::thread::LuaThreadMethods;
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

fn inject_globals(lua: &mlua::Lua) -> mlua::Result<()> {
    let task = lua_std::StandardLibrary::Task.into_lua(&lua)?;
    let web = lua_std::StandardLibrary::Web.into_lua(&lua)?;
    let co = lua.globals().get::<mlua::Table>("coroutine")?;

    co.set(
        "close",
        task.as_table().unwrap().get::<mlua::Function>("cancel")?,
    )?;

    lua.globals().set("task", task)?;

    lua.globals().set("web", web)?;

    lua.globals().set(
        "require",
        lua.create_async_function(lua_require::lua_require)?,
    )?;

    Ok(())
}

fn main() {
    let lua = mlua::Lua::new();

    inject_globals(&lua).expect("Failed to inject globals");

    scheduler::thread::initialize_threads(lua.clone(), |proxy| {
        lua.set_app_data(proxy);

        if let Err(err) = smol::block_on::<mlua::Result<()>>(async move {
            let script_path = std::env::args().nth(1).unwrap_or("init.luau".to_string());

            lua_require::utils::load_script(lua.clone(), PathBuf::from(script_path)).await?;

            Ok(())
        }) {
            eprintln!("{err}");
            std::process::exit(1);
        };
    });
}
