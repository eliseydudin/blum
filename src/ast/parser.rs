use super::{token::Keyword, AstError, Function, Lexer, Operand, Result, Token, TokenType};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i128),
    String(String),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Bin {
        left: Box<Expr>,
        right: Box<Expr>,
        operand: Operand,
    },
    Block {
        exprs: Vec<Expr>,
    },
    Function {
        name: String,
        params: HashMap<String, String>,
        rettype: String,
        body: Vec<Expr>,
    },
}

pub struct Parser {
    pub tokens: std::vec::IntoIter<Token>,
    pub ast: Vec<Expr>,
}

impl Parser {
    pub fn new(source: impl Into<String>) -> Self {
        let source: String = source.into();
        let source = source.chars();
        let mut lexer = Lexer::new(source);
        lexer.parse();
        let tokens = lexer.finish();
        let tokens = tokens.into_iter();

        let ast = Vec::new();

        Self { tokens, ast }
    }

    pub fn parse(&mut self) {
        for token in &mut self.tokens {
            match token.token_type {
                _ => (),
            };
        }
    }

    pub fn try_keyword(&mut self, keyword: Keyword) -> Result<Expr> {
        match keyword {
            Keyword::Fn => self.try_function(),
            _ => todo!(),
        }
    }

    pub fn try_function(&mut self) -> Result<Expr> {
        let identifier = self
            .tokens
            .next()
            .ok_or(AstError::Function(Function::NoIdentifier))?;

        let name = identifier
            .data
            .ok_or(AstError::Function(Function::NoIdentifier))?;

        let lparen = self
            .tokens
            .next()
            .ok_or(AstError::Function(Function::NoParenthesis))?;
        if lparen.token_type != TokenType::Operand(Operand::LParen) {
            return Err(AstError::Function(Function::NoParenthesis));
        }

        let params = self.try_function_params(Operand::RParen)?;
        let rettype = String::new();
        let body = vec![];

        let expr = Expr::Function {
            name,
            params,
            rettype,
            body,
        };

        Ok(expr)
    }

    pub fn try_function_next_parameter(&mut self) -> Result<(String, String)> {
        let param = self
            .tokens
            .next()
            .ok_or(AstError::Function(Function::ParamError))?;

        let param = if param.token_type == TokenType::Identifier {
            param.data.ok_or(AstError::Function(Function::ParamError))?
        } else {
            return Err(AstError::Function(Function::ParamError));
        };

        let colon = self
            .tokens
            .next()
            .ok_or(AstError::Function(Function::ParamError))?;

        if colon.token_type != TokenType::Operand(Operand::Colon) {
            return Err(AstError::Function(Function::ParamError));
        }

        let ptype = self
            .tokens
            .next()
            .ok_or(AstError::Function(Function::ParamError))?
            .data
            .ok_or(AstError::Function(Function::ParamError))?;

        Ok((param, ptype))
    }

    pub fn try_function_params(&mut self, end: Operand) -> Result<HashMap<String, String>> {
        let mut result = HashMap::new();

        loop {
            let next = self.try_function_next_parameter()?;
            result.insert(next.0, next.1);

            let next = self
                .tokens
                .next()
                .ok_or(AstError::Function(Function::ParamError))?;

            if let TokenType::Operand(op) = next.token_type {
                if op == end {
                    break;
                } else if op == Operand::Coma {
                    continue;
                } else {
                    return Err(AstError::Function(Function::ParamError));
                }
            } else {
                return Err(AstError::Function(Function::ParamError));
            }
        }

        Ok(result)
    }

    pub fn try_function_return(&mut self) -> Result<String> {
        self.await_token(Operand::Minus, || {
            AstError::Function(Function::NoReturnType)
        })?;
        self.await_token(Operand::More, || {
            AstError::Function(Function::ReturnTypeError)
        })?;

        todo!()
    }

    pub fn await_token(&mut self, op: Operand, f: fn() -> AstError) -> Result<Token> {
        match self.tokens.next() {
            Some(data) => {
                if data.token_type == TokenType::Operand(op) {
                    return Ok(data);
                } else {
                    return Err(f());
                }
            }
            None => return Err(f()),
        }
    }
}
