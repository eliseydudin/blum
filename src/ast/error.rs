use crate::ast::TokenType;
use std::{error::Error as ErrorTrait, fmt::Display};

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedToken {
        expected: TokenType,
        found: TokenType,
    },
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    position: usize,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "error (at {}): expected {expected:?}, found {found:?}",
                    self.position
                )
            }
        }
    }
}

impl ErrorTrait for Error {}

/*
() => {
    $crate::panicking::panic("not yet implemented")
};
($($arg:tt)+) => {
    $crate::panic!("not yet implemented: {}", $crate::format_args!($($arg)+))
};
*/
