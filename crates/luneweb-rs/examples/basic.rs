use luneweb_rs::classes::{eventloop::EventLoop, window::Window};
use mlua_luau_scheduler::Scheduler;

#[tokio::main]
async fn main() {
    let lua = mlua::Lua::new();
    let scheduler = Scheduler::new(&lua);

    EventLoop::new().finalize(&lua, &scheduler);

    Window::new(&lua)
        .with_title("window (1)")
        .with_webview()
        .finalize(&lua);

    Window::new(&lua)
        .with_title("window (2)")
        .with_webview()
        .finalize(&lua);

    scheduler.run().await;
}
