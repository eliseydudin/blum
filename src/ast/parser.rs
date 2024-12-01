use super::{Error, Expr, Lexer, TokenIter};

pub struct Parser {
    pub tokens: TokenIter,
    pub ast: Vec<Expr>,
}

impl Parser {
    pub fn new(buff: impl Into<String>) -> Self {
        let chars: String = buff.into();
        let tokens = Lexer::new(&chars).parse().finish();
        let tokens = TokenIter::new(tokens);
        let ast = vec![];

        Self { ast, tokens }
    }

    pub fn parse(&mut self) -> Vec<Error> {
        vec![]
    }
}
