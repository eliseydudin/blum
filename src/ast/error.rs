use std::{error::Error as ErrorTrait, fmt::Display};

use super::TokenType;

#[derive(Debug)]
pub enum Error {
    Todo,
    Message(String),
    Expect {
        expected: TokenType,
        found: TokenType,
    },
}

impl Error {
    pub fn new(s: impl Into<String>) -> Self {
        Self::Message(s.into())
    }

    pub const fn todo() -> Self {
        Self::Todo
    }

    pub fn wrap<T>(self) -> Result<T> {
        Err(self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Todo => write!(f, "error: not yet implemented"),
            Self::Expect { expected, found } => write!(
                f,
                "error: expected token \"{:?}\", found: \"{:?}\"",
                expected, found
            ),
            Self::Message(m) => write!(f, "error: {m}"),
        }
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
        $crate::ast::Error::todo()
    };
    ($msg:tt) => {
        $crate::ast::Error::new(format!("{}", $msg))
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
