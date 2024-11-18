use lua::*;
use mlua::prelude::*;
use scheduler::Scheduler;

// mod app;
pub mod app;
mod lua;
mod scheduler;

main!(|sched, proxy| {
    let lua = mlua::Lua::new();

    inject_globals(&lua).expect("Failed to inject globals");

    lua.set_app_data(sched.clone());
    lua.set_app_data(proxy.clone());

    sched
        .executor
        .spawn::<LuaResult<()>>(async move {
            let f = lua
                .load(smol::fs::read_to_string("app.luau").await?)
                .set_name("app.luau")
                .into_function()?;

            f.call_async::<()>(()).await?;

            Ok(())
        })
        .detach();
});
