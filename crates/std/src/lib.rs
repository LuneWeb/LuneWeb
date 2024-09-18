use luneweb_std_window::LuaWindow;
use mlua::IntoLua;

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

    pub fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {
            Self::Window => {
                let table = lua.create_table()?;
                table.set("new", mlua::Function::wrap(LuaWindow::new))?;
                table.into_lua(lua)
            }
        }
    }
}

/// Make sure to call this after Scheduler is created
pub fn inject_globals(lua: &mlua::Lua) -> mlua::Result<()> {
    let globals = lua.globals();

    globals.set("WindowBuilder", StandardLibrary::Window.into_lua(lua)?)?;

    Ok(())
}
