use luneweb_rs::classes::{eventloop::EventLoop, window::Window};
use mlua_luau_scheduler::Scheduler;

#[tokio::main]
async fn main() -> Result<(), String> {
    let lua = mlua::Lua::new();
    let scheduler = Scheduler::new(&lua);

    EventLoop::new().finalize(&lua, &scheduler);

    Window::new(&lua)?
        .with_title("window (1)")
        .with_webview(false, |x| x.with_url("https://roblox.com"))?
        .finalize(&lua)?;

    Window::new(&lua)?
        .with_title("window (2)")
        .with_webview(false, |x| x.with_url("https://luneweb.github.io/docs/"))?
        .finalize(&lua)?;

    scheduler.run().await;

    Ok(())
}
