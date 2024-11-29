use super::{Error, Expr, Lexer, Result, Token, TokenIter, TokenType};

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

    pub fn parse(&mut self) -> Vec<Error> {
        let mut errors = vec![];

        while let Some(token) = self.tokens.next() {
            match self.parse_next(token) {
                Ok(e) => self.ast.push(e),
                Err(e) => errors.push(e),
            }
        }

        errors
    }

    pub fn parse_next(&mut self, token: Token) -> Result<Expr> {
        match token.token_type {
            TokenType::Keyword(_) => self.try_keyword(token),
            _ => Ok(Expr::Todo),
        }
    }

    pub fn try_keyword(&mut self, token: Token) -> Result<Expr> {
        Ok(Expr::Todo)
    }
}
