#[cfg(feature = "std-audio")]
use luneweb_std_audio::{device::LuaAudioDevice, source::LuaAudioSource};
use luneweb_std_window::LuaWindow;
use mlua::IntoLua;

pub enum StandardLibrary {
    Window,
    #[cfg(feature = "std-audio")]
    Audio,
}

impl StandardLibrary {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<T: AsRef<str>>(str: &T) -> Option<Self> {
        match str.as_ref() {
            "window" => Some(Self::Window),
            #[cfg(feature = "std-audio")]
            "audio" => Some(Self::Audio),
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
            #[cfg(feature = "std-audio")]
            Self::Audio => {
                let table = lua.create_table()?;
                table.set(
                    "fromBuffer",
                    mlua::Function::wrap(LuaAudioSource::from_buffer),
                )?;
                table.set(
                    "fromBuffer",
                    mlua::Function::wrap(LuaAudioSource::from_buffer),
                )?;
                table.into_lua(lua)
            }
        }
    }
}

/// Make sure to call this after Scheduler is created
pub fn inject_globals(lua: &mlua::Lua) -> mlua::Result<()> {
    let globals = lua.globals();

    globals.set("WindowBuilder", StandardLibrary::Window.into_lua(lua)?)?;

    #[cfg(feature = "std-audio")]
    if LuaAudioDevice::try_init(lua).is_ok() {
        // only add AudioBuilder on supported machines
        globals.set("AudioBuilder", StandardLibrary::Audio.into_lua(lua)?)?;
    }

    Ok(())
}
