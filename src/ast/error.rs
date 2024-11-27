use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum Function {
    NoIdentifier,
    NoParenthesis,
    ParamError,
}

#[derive(Debug)]
pub enum AstError {
    Function(Function),
}

impl Display for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstError::Function(func) => write!(f, "error while parsing the function: {:?}", func),
        }
    }
}

impl Error for AstError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub type Result<T> = std::result::Result<T, AstError>;
