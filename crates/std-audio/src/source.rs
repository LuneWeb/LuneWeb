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
    pub content: BString,
    pub duration: f64,
}

impl LuaAudioSource {
    pub fn from_buffer(lua: &Lua, content: BString) -> LuaResult<Self> {
        let reader = BufReader::new(Cursor::new(content.clone()));
        let source = Decoder::new(reader).into_lua_err()?;
        let duration = source
            .total_duration()
            .ok_or(LuaError::runtime(
                "Audio length is either unknown or infinite",
            ))?
            .as_secs_f64();

        let sink = LuaAudioDevice::get(lua)?.sink()?;

        Ok(Self {
            sink,
            content,
            duration,
        })
    }
}

impl LuaUserData for LuaAudioSource {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("duration", |_, this| Ok(this.duration));

        fields.add_field_method_get("paused", |_, this| Ok(this.sink.is_paused()));
        fields.add_field_method_set("paused", |_, this, paused: bool| {
            if paused {
                this.sink.pause();
            } else {
                this.sink.play();
            }

            Ok(())
        });

        fields.add_field_method_get("position", |_, this| Ok(this.sink.get_pos().as_secs_f64()));
        fields.add_field_method_set("position", |_, this, pos: f64| {
            this.sink
                .try_seek(Duration::from_secs_f64(pos))
                .map_err(|x| LuaError::runtime(x.to_string()))?;

            Ok(())
        });

        fields.add_field_method_get("volume", |_, this| Ok(this.sink.volume()));
        fields.add_field_method_set("volume", |_, this, volume: f32| {
            this.sink.set_volume(volume);

            Ok(())
        });

        fields.add_field_method_get("speed", |_, this| Ok(this.sink.speed()));
        fields.add_field_method_set("speed", |_, this, speed: f32| {
            this.sink.set_speed(speed);

            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("play", |_, this, _: ()| {
            let reader = BufReader::new(Cursor::new(this.content.clone()));
            let source = Decoder::new(reader).into_lua_err()?;

            this.sink.clear();
            this.sink.append(source);
            this.sink.play();

            Ok(())
        });

        methods.add_method("stop", |_, this, _: ()| {
            this.sink.stop();

            Ok(())
        });
    }
}
