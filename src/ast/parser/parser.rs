use super::{ParserCause, ParserException};
use crate::ast::token::Token;
use crate::ast::{expr, statement as stmt, Lexer};

pub struct Parser {
    input: Vec<Token>,
    ast: Vec<stmt::Statement>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let input = lexer.lex();
        Self { input, ast: vec![] }
    }
}
