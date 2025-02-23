use std::io;

pub type UciResult<T> = Result<T, UciError>;

#[derive(Debug)]
pub enum UciError {
    IoError(io::Error),
    EngineError(String),
}

impl From<io::Error> for UciError {
    fn from(err: io::Error) -> Self {
        UciError::IoError(err)
    }
}
