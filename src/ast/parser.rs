use super::{Lexer, Token};

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(source: impl Into<String>) -> Self {
        let source: String = source.into();
        let source = source.chars();
        let mut lexer = Lexer::new(source);
        lexer.parse();
        let tokens = lexer.finish();

        Self { tokens }
    }
}
