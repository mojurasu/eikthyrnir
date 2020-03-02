use std::{fmt, error};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<git2::Error> for Error {
    fn from(_item: git2::Error) -> Self {
        Error
    }
}

impl From<regex::Error> for Error {
    fn from(_item: regex::Error) -> Self {
        Error
    }
}

impl From<std::io::Error> for Error {
    fn from(_item: std::io::Error) -> Self {
        Error
    }
}
