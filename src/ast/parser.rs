use super::{token::Keyword, AstError, Function, Lexer, Operand, Result, Token, TokenType};

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
        params: Vec<String>,
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

        let params = vec![];
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
}
