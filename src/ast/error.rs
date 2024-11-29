use std::{error::Error as ErrorTrait, fmt::Display};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(s: impl Into<String>) -> Self {
        Self { message: s.into() }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self.message)
    }
}

impl ErrorTrait for Error {
    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        None
    }
}

pub type Result<T> = std::result::Result<T, Error>;
