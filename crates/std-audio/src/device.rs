use mlua::{prelude::*, AppDataRef};
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct LuaAudioDevice {
    pub output: (OutputStream, OutputStreamHandle),
}

impl LuaAudioDevice {
    pub fn init(lua: &Lua) -> LuaResult<()> {
        match Self::new(lua, ()) {
            Ok(device) => {
                lua.set_app_data(device);
            }

            // give a warning instead of an error so the app can work
            // on machines that dont have audio devices (e.g. when using WSL)
            Err(_) => println!("[Warn] failed to get an audio device, errors should be expected when using the audio library"),
        }

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
