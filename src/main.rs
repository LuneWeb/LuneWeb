use luneweb::*;
use std::path::PathBuf;

fn inject_globals(lua: &mlua::Lua) -> mlua::Result<()> {
    let task = lua_std::StandardLibrary::Task.into_lua(&lua)?;
    let web = lua_std::StandardLibrary::Web.into_lua(&lua)?;
    let co = lua.globals().get::<mlua::Table>("coroutine")?;

    co.set(
        "close",
        task.as_table().unwrap().get::<mlua::Function>("cancel")?,
    )?;

    co.set(
        "resume",
        lua.create_async_function(
            |lua, (thread, args): (mlua::Thread, mlua::MultiValue)| async move {
                let proxy = lua.get_app_proxy();

                proxy.spawn_lua_thread(thread.clone(), Some(args));
                proxy.await_lua_thread(thread).await
            },
        )?,
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
