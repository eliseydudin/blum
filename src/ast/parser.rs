use std::collections::HashMap;

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
            .ok_or(Error::EOF(TokenType::Identifier))?;

        if !identifier.0 {
            return Error::Expect {
                expected: TokenType::Identifier,
                found: identifier.1.token_type,
            }
            .wrap();
        }

        let _lparen = self
            .tokens
            .expect_and_progress(Operand::LParen)
            .ok_or(Error::EOF(TokenType::Operand(Operand::LParen)))?;

        // we can do .unwrap since [`TokenType::Identifier`] always has some data
        let name = identifier.1.data.unwrap();
        let params = self.try_type_map()?;
        let rettype = self
            .try_function_return_type()?
            .unwrap_or("void".to_owned());
        let body = Box::new(self.try_block()?);

        Ok(Expr::Function {
            name,
            rettype,
            params,
            body,
        })
    }

    pub fn try_block(&mut self) -> Result<Expr> {
        todo!()
    }

    pub fn try_type_map(&mut self) -> Result<HashMap<String, String>> {
        todo!()
    }

    pub fn try_function_return_type(&mut self) -> Result<Option<String>> {
        todo!()
    }
}
