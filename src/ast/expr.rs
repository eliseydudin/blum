use super::token::TokenType;

#[derive(PartialEq, Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: TokenType,
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Int(i128),
    String(String),
    Float(f64),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Binary(Binary),
    Value(Value),
}
