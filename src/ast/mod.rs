pub mod lexer;
pub mod parser;
pub mod tokens;

pub use tokens::{Literal, Token, TokenType};

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    Let(Token, Option<Expr>),
    While(Expr, Box<Stmt>),
    Fn(Token, Vec<String>, Box<Stmt>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token),
}
