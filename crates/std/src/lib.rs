use luneweb_std_dom::LuaDom;
use luneweb_std_window::LuaWindow;

pub enum StandardLibrary {
    Window,
    Dom,
}

impl StandardLibrary {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<T: AsRef<str>>(str: &T) -> Option<Self> {
        match str.as_ref() {
            "window" => Some(Self::Window),
            "dom" => Some(Self::Dom),
            _ => None,
        }
    }

    pub fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
        let table = lua.create_table()?;

        match self {
            Self::Window => {
                table.set("new", mlua::Function::wrap(LuaWindow::new))?;
            }
            Self::Dom => {
                LuaDom::init_middleware(lua)?;
            }
        };

        Ok(table)
    }
}

/// Make sure to call this after Scheduler is created
pub fn inject_globals(lua: &mlua::Lua) -> mlua::Result<()> {
    let globals = lua.globals();

    globals.set("WindowBuilder", StandardLibrary::Window.into_lua(lua)?)?;
    globals.set("dom", StandardLibrary::Dom.into_lua(lua)?)?;

    Ok(())
}
