use crate::util::Error as LuneWebError;
use lune_std::context::GlobalsContextBuilder;
use std::fmt::Display;
use tao::error::OsError;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    Os(OsError),
    Custom(String),
    Wry(wry::Error),
    Mlua(mlua::Error),
}

impl From<OsError> for Error {
    fn from(value: OsError) -> Self {
        Self::Os(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

impl From<wry::Error> for Error {
    fn from(value: wry::Error) -> Self {
        Self::Wry(value)
    }
}

impl From<mlua::Error> for Error {
    fn from(value: mlua::Error) -> Self {
        Self::Mlua(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(str) => str.fmt(f),
            Error::Mlua(mlua) => mlua.fmt(f),
            Error::Os(os) => os.fmt(f),
            Error::Wry(wry) => wry.fmt(f),
        }
    }
}

/// Creates a GlobalsContextBuilder which contains lune's standard libraries
pub fn lune_ctx() -> Result<GlobalsContextBuilder, LuneWebError> {
    let mut builder = GlobalsContextBuilder::new();

    // Inject lune standard libraries
    lune_std::inject_libraries(&mut builder)?;

    Ok(builder)
}
