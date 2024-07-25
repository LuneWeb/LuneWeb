use mlua::Lua;

pub const LUA_SERDE_CONFIG: lune_std_serde::EncodeDecodeConfig =
    lune_std_serde::EncodeDecodeConfig {
        format: lune_std_serde::EncodeDecodeFormat::Json,
        pretty: false,
    };

pub fn lua_to_json<'lua>(
    value: mlua::Value<'lua>,
    lua: &'lua Lua,
) -> Result<mlua::String<'lua>, mlua::Error> {
    lune_std_serde::encode(value, lua, LUA_SERDE_CONFIG)
}

pub fn json_to_lua<T: Into<String>>(json: T, lua: &Lua) -> Result<mlua::Value, mlua::Error> {
    lune_std_serde::decode(json.into(), lua, LUA_SERDE_CONFIG)
}
