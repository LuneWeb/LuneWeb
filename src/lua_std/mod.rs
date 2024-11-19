use crate::app::AppProxy;

mod task;
mod web;

pub enum StandardLibrary {
    Task,
    Web,
}

impl StandardLibrary {
    pub fn into_lua(self, lua: &mlua::Lua, proxy: &AppProxy) -> mlua::Result<mlua::Value> {
        match self {
            Self::Task => task::create(lua, proxy),
            Self::Web => web::create(lua, proxy),
        }
    }
}
