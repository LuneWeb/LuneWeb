use crate::device::LuaAudioDevice;
use bstr::BString;
use mlua::prelude::*;
use rodio::{Decoder, Sink};
use std::{
    io::{BufReader, Cursor},
    time::Duration,
};

pub struct LuaAudioSource {
    pub sink: Sink,
}

impl LuaAudioSource {
    pub fn new(lua: &Lua, buffer: BString) -> LuaResult<Self> {
        let reader = BufReader::new(Cursor::new(buffer));
        let source = Decoder::new(reader).into_lua_err()?;
        let sink = LuaAudioDevice::get(lua)?.sink()?;

        sink.append(source);

        Ok(Self { sink })
    }
}

impl LuaUserData for LuaAudioSource {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("position", |_, this| Ok(this.sink.get_pos().as_secs_f64()));
        fields.add_field_method_set("position", |_, this, pos: f64| {
            this.sink
                .try_seek(Duration::from_secs_f64(pos))
                .map_err(|x| LuaError::runtime(x.to_string()))?;

            Ok(())
        });
    }
}
