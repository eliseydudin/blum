use super::{
    collect_to, token::Keyword, AstError, Collect, Function, Lexer, Operand, Result, Token,
    TokenType,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i128),
    String(String),
}

#[derive(Clone, Debug)]
pub enum Expr {
    // a <op> b
    Bin {
        left: Box<Expr>,
        right: Box<Expr>,
        operand: Operand,
    },
    // { ... }
    Block {
        exprs: Vec<Expr>,
    },
    // fn <ident>(<param>: <ptype>, ...) -> <rettype> { ... }
    Function {
        name: String,
        params: HashMap<String, String>,
        rettype: String,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: std::vec::IntoIter<Token>,
    pub ast: Vec<Expr>,
    pub errors: Vec<AstError>,
}

impl Collect for Parser {
    fn collect<T>(&mut self, err: Result<T>) {
        match err {
            Ok(_) => (),
            Err(e) => self.errors.push(e),
        }
    }
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
        let errors = Vec::new();

        Self {
            tokens,
            ast,
            errors,
        }
    }

    pub fn parse(&mut self) {
        let mut clone = self.clone();
        for token in &mut self.tokens {
            match token.token_type {
                TokenType::Error(err) => eprintln!("error: {err}"),
                TokenType::Keyword(kw) => {
                    match kw {
                        Keyword::Fn => {
                            let err = clone.try_function();
                            collect_to(err, &mut clone);
                        }
                        _ => (),
                    };
                }
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
        let identifier = self.await_token(TokenType::Identifier, || {
            AstError::Function(Function::NoIdentifier)
        })?;

        let name = identifier
            .data
            .ok_or(AstError::Function(Function::NoIdentifier))?;

        self.await_token(Operand::LParen, || {
            AstError::Function(Function::NoParenthesis)
        })?;

        let params = self.try_function_params(Operand::RParen)?;
        let rettype = self.try_function_return()?;
        let body = Box::new(self.try_block()?);

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
            .await_token(TokenType::Identifier, || {
                AstError::Function(Function::ParamError)
            })?
            .data
            .ok_or(AstError::Function(Function::ParamError))?;

        self.await_token(Operand::Colon, || AstError::Function(Function::ParamError))?;

        let ptype = self
            .await_token(TokenType::Identifier, || {
                AstError::Function(Function::ParamError)
            })?
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
        let rettype = self
            .await_token(TokenType::Identifier, || {
                AstError::Function(Function::ReturnTypeError)
            })?
            .data
            .ok_or(AstError::Function(Function::ReturnTypeError))?;

        Ok(rettype)
    }

    pub fn await_token(&mut self, op: impl Into<TokenType>, f: fn() -> AstError) -> Result<Token> {
        let ttype: TokenType = op.into();

        match self.tokens.next() {
            Some(data) => {
                if data.token_type == ttype {
                    return Ok(data);
                } else {
                    return Err(f());
                }
            }
            None => return Err(f()),
        }
    }

    pub fn try_block(&mut self) -> Result<Expr> {
        todo!()
    }
}
