pub mod app;
pub mod lua_bindings;
pub mod lua_require;
pub mod lua_std;
pub mod scheduler;
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
