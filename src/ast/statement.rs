use super::expr::Expr;

pub struct Let {
    pub name: String,
    pub value: Expr,
}

pub struct Block {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Let(Let),
    Block(Block),
}
