use http::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};
use mlua::prelude::*;
use std::str::FromStr;

pub mod request;
pub mod response;

pub fn lua_table_to_headers<'lua>(
    headers: Option<LuaTable<'lua>>,
    _: &'lua Lua,
) -> LuaResult<HeaderMap> {
    let mut headers_map = HeaderMap::new();

    if let Some(headers) = headers {
        for pair in headers.pairs::<String, LuaString>() {
            let (h, v) = pair?;
            let name = HeaderName::from_str(&h).into_lua_err()?;
            let value = HeaderValue::from_bytes(v.as_bytes()).into_lua_err()?;
            headers_map.append(name, value);
        }
    }

    Ok(headers_map)
}
