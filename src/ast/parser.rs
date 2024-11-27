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
    Todo, // remove later when the parser is complete
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize, // the current used to access the elements
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

        let ast = Vec::new();
        let errors = Vec::new();
        let current = 0usize;

        Self {
            tokens,
            ast,
            errors,
            current,
        }
    }

    pub fn parse(&mut self) {
        while self.current < self.tokens.len() {
            let token = &self.tokens[self.current];
            match token.token_type {
                TokenType::Error(err) => eprintln!("error: {err}"),
                TokenType::Keyword(kw) => {
                    let expr = self.try_keyword(kw);
                    match expr {
                        Ok(data) => self.ast.push(data),
                        Err(_) => collect_to(expr, self),
                    };
                }
                _ => (),
            };

            self.current += 1;
        }
    }

    pub fn try_keyword(&mut self, keyword: Keyword) -> Result<Expr> {
        let expr = match keyword {
            Keyword::Fn => {
                let res = self.try_function()?;
                res
            }
            _ => Expr::Todo,
        };

        Ok(expr)
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

        let params = self
            .try_function_params(Operand::RParen)
            .unwrap_or(HashMap::new());

        let rettype = match self.try_function_return() {
            Ok(data) => data,
            Err(e) => {
                if e == AstError::Function(Function::NoReturnType) {
                    "void".to_owned()
                } else {
                    return Err(e);
                }
            }
        };
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
        let tk = self.next();

        match tk {
            Some(data) => {
                if data.token_type == ttype {
                    return Ok(data.clone());
                } else {
                    return Err(f());
                }
            }
            None => return Err(f()),
        }
    }

    pub fn try_block(&mut self) -> Result<Expr> {
        let block = Expr::Block { exprs: vec![] };
        while let Some(n) = self.next() {
            if n.token_type == TokenType::Operand(Operand::RFigure) {
                break;
            }
        }
        Ok(block)
    }

    pub fn next(&mut self) -> Option<Token> {
        self.current += 1;
        self.tokens.get(self.current).cloned()
    }
}
