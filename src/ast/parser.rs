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

        // _lparen should just be ignored
        let _lparen = self
            .tokens
            .expect_and_progress(Operand::LParen)
            .ok_or(Error::EOF(TokenType::Operand(Operand::LParen)))?;

        // we can do .unwrap since [`TokenType::Identifier`] always has some data
        let name = identifier.1.data.unwrap();
        let params = self.try_type_map(Operand::RParen)?;
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
        eprintln!("warning! `try_block` currently does nothing!");
        while let Some(next) = self.tokens.next() {
            if next.token_type == Operand::RFigure.into() {
                break;
            }
        }

        Ok(Expr::Block(vec![]))
    }

    pub fn try_type_map(&mut self, end: Operand) -> Result<HashMap<String, String>> {
        eprintln!("warning! `try_type_map` currently does nothing!");
        while let Some(next) = self.tokens.next() {
            if next.token_type == end.into() {
                break;
            }
        }

        if self.tokens.peek().is_none() {
            return error!("try_type_map found EOF!").wrap();
        }

        let result = HashMap::new();
        Ok(result)
    }

    pub fn try_function_return_type(&mut self) -> Result<Option<String>> {
        let token = self.tokens.current().ok_or(error!("found EOF!"))?;

        if token.token_type == Operand::LFigure.into() {
            return Ok(None);
        } else if token.token_type == Operand::Minus.into() {
            self.tokens.progress();
            let more_token = self.tokens.next().ok_or(error!("found EOF!"))?;
            let type_identifier = self.tokens.next().ok_or(error!("found EOF!"))?;

            if more_token.token_type == Operand::More.into()
                && type_identifier.token_type == TokenType::Identifier
            {
                return Ok(Some(type_identifier.data.unwrap()));
            } else {
                return error!("unexpected token found in the -> operator!").wrap();
            }
        }

        error!().wrap()
    }
}
