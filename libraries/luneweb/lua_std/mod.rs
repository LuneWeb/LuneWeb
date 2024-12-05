mod task;
mod web;

pub enum StandardLibrary {
    Task,
    Web,
}

impl StandardLibrary {
    pub fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {
            Self::Task => task::create(lua),
            Self::Web => web::create(lua),
        }
    }
}
