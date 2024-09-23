use bstr::BString;
use mlua::prelude::*;
use rodio::Decoder;
use std::io::{BufReader, Cursor};

pub struct LuaAudioSource {
    pub source: Decoder<BufReader<Cursor<BString>>>,
}

impl LuaAudioSource {
    pub fn new(_: &Lua, buffer: BString) -> LuaResult<Self> {
        let reader = BufReader::new(Cursor::new(buffer));
        let source = Decoder::new(reader).into_lua_err()?;

        Ok(Self { source })
    }
}

impl LuaUserData for LuaAudioSource {}
