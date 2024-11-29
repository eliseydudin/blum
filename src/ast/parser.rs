use super::{Lexer, Token, TokenIter};

pub struct Parser {
    pub tokens: TokenIter,
    pub ast: Vec<()>,
}

impl Parser {
    pub fn new(input: impl Into<String>) -> Self {
        let input: String = input.into();

        let tokens = Lexer::new(input.chars()).parse().finish();
        let tokens = TokenIter::new(tokens);
        let ast = vec![];

        Self { ast, tokens }
    }
}
