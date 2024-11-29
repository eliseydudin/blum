use super::{Expr, Lexer, Result, Token, TokenIter, TokenType};

pub struct Parser {
    pub tokens: TokenIter,
    pub ast: Vec<Expr>,
}

impl Parser {
    pub fn new(input: impl Into<String>) -> Self {
        let input: String = input.into();

        let tokens = Lexer::new(input.chars()).parse().finish();
        let tokens = TokenIter::new(tokens);
        let ast = vec![];

        Self { ast, tokens }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.tokens.next() {
            self.parse_next(token);
        }
    }

    pub fn parse_next(&mut self, token: Token) {
        match token.token_type {
            TokenType::Keyword(_) => (),
            _ => (),
        }
    }

    pub fn try_keyword(&mut self, token: Token) -> Result<Expr> {
        todo!()
    }
}
