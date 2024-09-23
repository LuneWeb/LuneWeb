use bstr::BString;
use mlua::prelude::*;
use rodio::{Decoder, Sink};
use std::io::{BufReader, Cursor};

use crate::device::LuaAudioDevice;

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

impl LuaUserData for LuaAudioSource {}
