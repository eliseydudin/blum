use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    NoIdentifier,
    NoParenthesis,
    ParamError,
    NoReturnType,
    ReturnTypeError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstError {
    Function(Function),
    WTF, // if this was returned something bad has happened
}

impl Display for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstError::Function(func) => write!(f, "error while parsing the function: {:?}", func),
            AstError::WTF => write!(f, "HOW DID WE GET HERE"),
        }
    }
}

impl Error for AstError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub type Result<T> = std::result::Result<T, AstError>;
