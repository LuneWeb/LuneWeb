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
pub fn inject_globals(lua: &mlua::Lua, sandbox: bool) -> mlua::Result<()> {
    let globals = lua.globals();

    globals.set("WindowBuilder", StandardLibrary::Window.into_lua(lua)?)?;

    for lib in lune_std::LuneStandardLibrary::ALL {
        let module = lib.module(lua)?;
        globals.set(lib.name(), &module[0])?;
    }

    for lib in lune_std::LuneStandardGlobal::ALL {
        globals.set(lib.name(), lib.create(lua)?)?;
    }

    if sandbox {
        lua.sandbox(true)?;

        lua.globals().set(
            lune_std::LuneStandardGlobal::GTable.name(),
            lune_std::LuneStandardGlobal::GTable.create(lua)?,
        )?;
    }

    Ok(())
}
