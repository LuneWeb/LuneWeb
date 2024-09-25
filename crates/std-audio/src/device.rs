use mlua::{prelude::*, AppDataRef};
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct LuaAudioDevice {
    pub output: (OutputStream, OutputStreamHandle),
}

impl LuaAudioDevice {
    pub fn try_init(lua: &Lua) -> LuaResult<()> {
        lua.set_app_data(Self::new(lua, ())?);

        Ok(())
    }

    pub fn get(lua: &Lua) -> LuaResult<AppDataRef<Self>> {
        lua.app_data_ref::<LuaAudioDevice>()
            .ok_or(LuaError::runtime(
                "Failed to get AudioDevice from app data container",
            ))
    }

    pub fn new(_: &Lua, _: ()) -> LuaResult<Self> {
        let output = OutputStream::try_default().into_lua_err()?;

        Ok(Self { output })
    }

    pub fn sink(&self) -> LuaResult<Sink> {
        Sink::try_new(&self.output.1).into_lua_err()
    }
}
