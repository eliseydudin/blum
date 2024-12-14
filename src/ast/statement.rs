use super::expr::Expr;

#[derive(PartialEq, Debug)]
pub struct Let {
    pub name: String,
    pub value: Expr,
}

#[derive(PartialEq, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(Let),
    Block(Block),
}
