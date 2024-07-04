use tao::error::OsError;

#[derive(Debug)]
pub enum Error {
    OsError(OsError),
    String(String),
}

impl From<OsError> for Error {
    fn from(value: OsError) -> Self {
        Self::OsError(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
