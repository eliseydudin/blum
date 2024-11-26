use super::{Lexer, Operand, Token, TokenType};

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
    Tuple {
        exprs: Vec<Expr>,
    },
    Value(Value),
    VarRef(String),
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
}
