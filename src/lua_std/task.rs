use crate::{app::AppProxy, scheduler, utils::table_builder::TableBuilder};
use mlua::IntoLua;

pub(super) fn create(lua: &mlua::Lua, proxy: &AppProxy) -> mlua::Result<mlua::Value> {
    let spawn_proxy = proxy.clone();
    let defer_proxy = proxy.clone();

    TableBuilder::new(lua)?
        .with_async_function("wait", |_, secs: Option<f64>| async move {
            if let Some(secs) = secs {
                smol::Timer::after(std::time::Duration::from_secs_f64(secs)).await;
            } else {
                smol::future::yield_now().await;
            }

            Ok(())
        })?
        .with_function(
            "spawn",
            move |lua, (f, args): (mlua::Function, mlua::MultiValue)| {
                let thread = lua.create_thread(f)?;

                if scheduler::thread::process_lua_thread(&thread, Some(args)) {
                    spawn_proxy.spawn_lua_thread(thread, None);
                };

                Ok(())
            },
        )?
        .with_function(
            "defer",
            move |lua, (f, args): (mlua::Function, mlua::MultiValue)| {
                let thread = lua.create_thread(f)?;

                defer_proxy.spawn_lua_thread(thread, Some(args));

                Ok(())
            },
        )?
        .build_readonly()?
        .into_lua(lua)
}
