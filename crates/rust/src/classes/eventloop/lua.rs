use std::time::Duration;

use super::EventLoop;

/// NOTE: increase the duration if the application crashes regularly
const INTERVAL: Duration = Duration::from_millis(4);

pub async fn lua_run(lua: &mlua::Lua, _: ()) -> mlua::Result<()> {
    loop {
        let Some(mut event_loop) = lua.app_data_mut::<EventLoop>() else {
            continue;
        };

        event_loop.once();

        if !event_loop.windows.is_empty()
            && event_loop
                .windows
                .iter()
                .all(|window| !window.inner.is_visible())
        {
            // All windows are closed
            event_loop.once();
            break;
        }

        // drop mutable reference before using await
        drop(event_loop);

        tokio::time::sleep(INTERVAL).await;
    }

    std::process::exit(0)
}
