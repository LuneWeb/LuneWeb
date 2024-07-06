use tao::error::OsError;

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
