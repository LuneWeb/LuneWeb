use scheduler::Scheduler;

pub mod app;
mod scheduler;

pub const ALWAYS_SINGLE_THREAD: bool = false;
pub const WINDOW_DEFAULT_TITLE: &str = "LuauApp";

main!(|sched, proxy, lua| -> mlua::Result<()> {
    let thread = lua.create_thread(
        lua.load(smol::fs::read_to_string("app.luau").await?)
            .set_name("app.luau")
            .into_function()?,
    )?;

    lua.globals().set(
        "wait",
        lua.create_async_function(|_, secs: Option<f64>| async move {
            if let Some(secs) = secs {
                smol::Timer::after(std::time::Duration::from_secs_f64(secs)).await;
            } else {
                smol::future::yield_now().await;
            }

            Ok(())
        })?,
    )?;

    let proxy_inner = proxy.clone();
    lua.globals().set(
        "spawn",
        lua.create_function(move |lua, (f, args): (mlua::Function, mlua::MultiValue)| {
            let thread = lua.create_thread(f)?;

            if scheduler::thread::process_lua_thread(&thread, Some(args)) {
                proxy_inner.spawn_lua_thread(thread);
            };

            Ok(())
        })?,
    )?;

    proxy.spawn_lua_thread(thread);

    let window = proxy.create_window(Some("Application".to_owned()));
    println!("{window:?}");

    Ok(())
});
