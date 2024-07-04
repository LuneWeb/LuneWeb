use tao::error::OsError;

#[derive(Debug)]
pub enum Error {
    Os(OsError),
    Custom(String),
    Wry(wry::Error),
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
