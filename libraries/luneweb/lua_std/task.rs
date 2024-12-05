use crate::{app::LuaThread, scheduler, utils::table_builder::TableBuilder, LuaAppProxyMethods};
use mlua::IntoLua;

pub(super) fn create(lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
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

                if let Some(result) = scheduler::thread::process_lua_thread(&thread, Some(args)) {
                    lua.get_app_proxy()
                        .proxy
                        .send_event(crate::app::AppEvent::StoreLuaThread {
                            thread: LuaThread::new(thread.clone(), result),
                        })
                        .expect("Failed to send thread result to scheduler");
                } else {
                    lua.get_app_proxy().spawn_lua_thread(thread.clone(), None);
                };

                Ok(thread)
            },
        )?
        .with_function(
            "defer",
            move |lua, (f, args): (mlua::Function, mlua::MultiValue)| {
                let thread = lua.create_thread(f)?;

                lua.get_app_proxy()
                    .spawn_lua_thread(thread.clone(), Some(args));

                Ok(thread)
            },
        )?
        .with_function("cancel", move |lua, thread: mlua::Thread| {
            let proxy = lua.get_app_proxy();
            proxy.cancel_lua_thread(thread);

            Ok(())
        })?
        .build_readonly()?
        .into_lua(lua)
}
