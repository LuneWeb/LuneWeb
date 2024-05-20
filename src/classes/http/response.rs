use super::lua_table_to_headers;
use bstr::{BString, ByteSlice};
use http::{HeaderMap, Response};
use mlua::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy)]
pub enum LuaResponseKind {
    PlainText,
    Table,
}

pub struct LuaResponse {
    pub kind: LuaResponseKind,
    pub status: u16,
    pub headers: HeaderMap,
    pub body: Option<Vec<u8>>,
}

pub trait IntoResponseBody {
    type BodyType;

    fn into_response_body(body: Vec<u8>) -> Self::BodyType;
}

impl IntoResponseBody for Cow<'static, [u8]> {
    type BodyType = Cow<'static, [u8]>;

    fn into_response_body(body: Vec<u8>) -> Self::BodyType {
        Cow::Owned(body)
    }
}

impl LuaResponse {
    pub fn into_response<B: IntoResponseBody>(self) -> LuaResult<Response<B::BodyType>> {
        Ok(match self.kind {
            LuaResponseKind::PlainText => Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(B::into_response_body(self.body.unwrap()))
                .into_lua_err()?,
            LuaResponseKind::Table => {
                let mut response = Response::builder()
                    .status(self.status)
                    .body(B::into_response_body(self.body.unwrap()))
                    .into_lua_err()?;
                response.headers_mut().extend(self.headers);
                response
            }
        })
    }
}

impl FromLua<'_> for LuaResponse {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        match value {
            // Plain strings from the handler are plaintext responses
            LuaValue::String(s) => Ok(Self {
                kind: LuaResponseKind::PlainText,
                status: 200,
                headers: HeaderMap::new(),
                body: Some(s.as_bytes().to_vec()),
            }),
            // Tables are more detailed responses with potential status, headers, body
            LuaValue::Table(t) => {
                let status: Option<u16> = t.get("status")?;
                let headers: Option<LuaTable> = t.get("headers")?;
                let body: Option<BString> = t.get("body")?;

                let headers_map = lua_table_to_headers(headers, lua)?;
                let body_bytes = body.map(|s: BString| s.as_bytes().to_vec());

                Ok(Self {
                    kind: LuaResponseKind::Table,
                    status: status.unwrap_or(200),
                    headers: headers_map,
                    body: body_bytes,
                })
            }
            // Anything else is an error
            value => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "NetServeResponse",
                message: None,
            }),
        }
    }
}
