use crate::error;

use super::{token::Keyword, Error, Expr, Lexer, Operand, Result, Token, TokenIter, TokenType};

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
            TokenType::Keyword(kw) => self.try_keyword(kw),
            _ => error!().wrap(),
        }
    }

    pub fn try_keyword(&mut self, kw: Keyword) -> Result<Expr> {
        match kw {
            Keyword::Fn => self.try_function(),
            _ => error!().wrap(),
        }
    }

    pub fn try_function(&mut self) -> Result<Expr> {
        let identifier = self
            .tokens
            .expect_and_progress(TokenType::Identifier)
            .ok_or(error!("expected identifier!"))?;

        if !identifier.0 {
            return Error::Expect {
                expected: TokenType::Identifier,
                found: identifier.1.token_type,
            }
            .wrap();
        }

        todo!()
    }

    pub fn try_block(&mut self) -> Result<Expr> {
        todo!()
    }
}
