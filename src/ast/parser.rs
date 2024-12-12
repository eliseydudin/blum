use super::{expr, statement as stmt, token as tk, token_stream::TokenStream};

pub struct Parser {
    tokens: Vec<tk::Token>,
    current: usize,
    statements: Vec<stmt::Statement>,
}

impl Parser {
    pub fn new(lexer: TokenStream) -> Self {
        let tokens = lexer.lex();
        Self {
            tokens,
            current: 0,
            statements: vec![],
        }
    }

    pub fn parse(self) -> Vec<stmt::Statement> {
        self.statements
    }

    pub fn peek(&self) -> Option<tk::Token> {
        self.tokens.get(self.current + 1).cloned()
    }

    pub fn advance(&mut self) -> Option<tk::Token> {
        self.current += 1;
        self.tokens.get(self.current - 1).cloned()
    }
}
