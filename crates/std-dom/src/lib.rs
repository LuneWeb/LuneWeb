use luneweb_rs::classes::webview::Middlewares;

pub struct LuaDom {}

impl LuaDom {
    pub fn init_middleware(lua: &mlua::Lua) -> mlua::Result<()> {
        Middlewares::add_middleware(lua, include_str!("middleware.js"))
    }
}
