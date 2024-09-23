use crate::device::LuaAudioDevice;
use bstr::BString;
use mlua::prelude::*;
use rodio::{Decoder, Sink, Source};
use std::{
    io::{BufReader, Cursor},
    time::Duration,
};

pub struct LuaAudioSource {
    pub sink: Sink,
    pub duration: f64,
}

impl LuaAudioSource {
    pub fn new(lua: &Lua, buffer: BString) -> LuaResult<Self> {
        let reader = BufReader::new(Cursor::new(buffer));
        let source = Decoder::new(reader).into_lua_err()?;
        let duration = source
            .total_duration()
            .ok_or(LuaError::runtime(
                "Audio length is either unknown or infinite",
            ))?
            .as_secs_f64();

        let sink = LuaAudioDevice::get(lua)?.sink()?;
        sink.append(source);

        Ok(Self { sink, duration })
    }
}

impl LuaUserData for LuaAudioSource {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("duration", |_, this| Ok(this.duration));

        fields.add_field_method_get("position", |_, this| Ok(this.sink.get_pos().as_secs_f64()));
        fields.add_field_method_set("position", |_, this, pos: f64| {
            this.sink
                .try_seek(Duration::from_secs_f64(pos))
                .map_err(|x| LuaError::runtime(x.to_string()))?;

            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("play", |_, this, _: ()| {
            this.sink.play();

            Ok(())
        });

        methods.add_method("pause", |_, this, _: ()| {
            this.sink.pause();

            Ok(())
        });

        methods.add_method("stop", |_, this, _: ()| {
            this.sink.stop();

            Ok(())
        });
    }
}
