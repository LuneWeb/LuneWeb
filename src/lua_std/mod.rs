use crate::app::AppProxy;

mod task;

pub enum StandardLibrary {
    Task,
}

impl StandardLibrary {
    pub fn into_lua(self, lua: &mlua::Lua, proxy: &AppProxy) -> mlua::Result<mlua::Value> {
        match self {
            Self::Task => task::create(lua, proxy),
        }
    }
}
