use super::{Lexer, Token};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub ast: Vec<()>,
    pub position: usize,
}

impl Parser {
    pub fn new(input: impl Into<String>) -> Self {
        let input: String = input.into();

        let tokens = Lexer::new(input.chars()).parse().finish();
        let ast = vec![];
        let position = 0usize;

        Self {
            tokens,
            ast,
            position,
        }
    }
}
