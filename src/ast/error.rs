use super::{Token, TokenType};
use std::{error::Error as ErrorTrait, fmt::Display, process::Output};

#[derive(Debug)]
pub enum Error {
    Todo,
    Message(String),
    Expect {
        expected: TokenType,
        found: TokenType,
    },
    EOF(TokenType),
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
            Self::EOF(e) => write!(f, "error: expected {e:?}, found end of file"),
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

pub trait ExpectUtils {
    type Output;

    fn expect_ext(&self, expected: TokenType) -> Result<Self::Output>;
}

impl ExpectUtils for Option<(bool, Token)> {
    type Output = Token;

    fn expect_ext(&self, expected: TokenType) -> Result<Self::Output> {
        if let Some(data) = self {
            return if data.0 {
                Ok(data.1.clone())
            } else {
                Error::Expect {
                    expected,
                    found: data.1.token_type.clone(),
                }
                .wrap()
            };
        }

        Error::EOF(expected).wrap()
    }
}

pub trait EofFoundUtils {
    type Output;

    fn eof_error(self) -> Result<Self::Output>;
}

impl EofFoundUtils for Option<Token> {
    type Output = Token;

    fn eof_error(self) -> Result<Self::Output> {
        match self {
            Some(t) => Ok(t),
            None => error!("found unexpected EOF!").wrap(),
        }
    }
}

/*
() => {
    $crate::panicking::panic("not yet implemented")
};
($($arg:tt)+) => {
    $crate::panic!("not yet implemented: {}", $crate::format_args!($($arg)+))
};
*/
