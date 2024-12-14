use crate::{ast::token::TokenType, error::Exception};
use std::{error, fmt};

#[derive(Debug)]
pub enum ParserCause {
    Expected(TokenType, TokenType),
}

impl fmt::Display for ParserCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expected(expected, found) => {
                write!(f, "expected `{expected:?}`, found: `{found:?}`")
            }
        }
    }
}

#[derive(Debug)]
pub struct ParserException {
    pos: (usize, usize),
    cause: ParserCause,
}

impl ParserException {
    pub fn expected_error(expected: TokenType, found: TokenType, pos: (usize, usize)) -> Self {
        let cause = ParserCause::Expected(expected, found);
        Self { cause, pos }
    }
}

impl error::Error for ParserException {}

impl fmt::Display for ParserException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl Exception for ParserException {
    fn at(&self) -> (usize, usize) {
        self.pos
    }
}
