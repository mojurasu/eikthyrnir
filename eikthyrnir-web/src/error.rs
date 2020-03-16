use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Git(git2::Error),
    Regex(regex::Error),
}

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

impl From<io::Error> for Error {
    fn from(item: io::Error) -> Self {
        Error::IO(item)
    }
}

impl From<git2::Error> for Error {
    fn from(item: git2::Error) -> Self {
        Error::Git(item)
    }
}

impl From<regex::Error> for Error {
    fn from(item: regex::Error) -> Self {
        Error::Regex(item)
    }
}
