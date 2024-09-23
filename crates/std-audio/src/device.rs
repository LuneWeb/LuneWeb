use crate::source::LuaAudioSource;
use mlua::prelude::*;
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct LuaAudioDevice {
    pub output: (OutputStream, OutputStreamHandle),
    pub sink: Sink,
}

impl LuaAudioDevice {
    pub fn new(_: &Lua, _: ()) -> LuaResult<Self> {
        let output = OutputStream::try_default().into_lua_err()?;
        let sink = Sink::try_new(&output.1).into_lua_err()?;

        Ok(Self { output, sink })
    }
}

impl LuaUserData for LuaAudioDevice {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Len, |_, this, _: ()| Ok(this.sink.len()));

        methods.add_method("appendAudioSource", |_, this, source: LuaAnyUserData| {
            let source = source.take::<LuaAudioSource>()?;
            this.sink.append(source.source);

            Ok(())
        });

        methods.add_method("play", |_, this, _: ()| {
            this.sink.play();

            Ok(())
        });

        methods.add_method("pause", |_, this, _: ()| {
            this.sink.pause();

            Ok(())
        });

        methods.add_method("clear", |_, this, _: ()| {
            this.sink.clear();

            Ok(())
        });
    }
}
