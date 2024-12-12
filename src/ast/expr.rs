use super::token::TokenType;

pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: TokenType,
}

pub enum Expr {
    Binary(Binary),
}
