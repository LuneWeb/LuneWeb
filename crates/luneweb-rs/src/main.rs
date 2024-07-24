use luneweb_rs::classes::{eventloop::EventLoop, window::Window};

fn main() {
    let lua = mlua::Lua::new();

    EventLoop::new().finalize(&lua);

    Window::new(&lua)
        .with_title("window (1)")
        .with_webview()
        .finalize(&lua);

    Window::new(&lua)
        .with_title("window (2)")
        .with_webview()
        .finalize(&lua);
}
