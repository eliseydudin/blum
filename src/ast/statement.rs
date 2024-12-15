use super::expr::Expr;

#[derive(PartialEq, Debug)]
#[non_exhaustive]
pub struct Let {
    pub name: String,
    pub value: Expr,
}

#[derive(PartialEq, Debug)]
#[non_exhaustive]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(PartialEq, Debug)]
#[non_exhaustive]
pub enum Statement {
    Let(Let),
    Block(Block),
}
