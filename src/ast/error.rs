use std::{error::Error as ErrorTrait, fmt::Display};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(s: impl Into<String>) -> Self {
        Self { message: s.into() }
    }

    pub fn wrap<T>(self) -> Result<T> {
        Err(self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ErrorTrait for Error {
    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        None
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! error {
    () => {
        $crate::ast::Error::new("error: not yet implemented")
    };
    ($($arg:tt)+) => {
        $crate::Error::new("error: {}", format!($($arg)+))
    };
}

/*
() => {
    $crate::panicking::panic("not yet implemented")
};
($($arg:tt)+) => {
    $crate::panic!("not yet implemented: {}", $crate::format_args!($($arg)+))
};
*/
