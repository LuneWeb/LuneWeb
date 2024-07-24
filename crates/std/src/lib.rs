use luneweb_std_window::LuaWindow;

pub enum StandardLibrary {
    Window,
}

impl StandardLibrary {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<T: AsRef<str>>(str: &T) -> Option<Self> {
        match str.as_ref() {
            "window" => Some(Self::Window),
            _ => None,
        }
    }

    pub fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
        let table = lua.create_table()?;

        match self {
            Self::Window => table.set("new", mlua::Function::wrap(LuaWindow::new)),
        }?;

        Ok(table)
    }
}

/// Make sure to call this after Scheduler is created
pub fn inject_globals(lua: &mlua::Lua, table: &mlua::Table) -> mlua::Result<()> {
    table.set("WindowBuilder", StandardLibrary::Window.into_lua(lua)?)?;
    table.set("task", lune_std_task::module(lua)?)?;

    Ok(())
}
