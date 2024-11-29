use std::collections::HashMap;

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Div,
    Mult,
}

#[derive(Debug)]
pub enum Expr {
    BinOp {
        op: BinOp,
        lhand: Box<Expr>,
        rhand: Box<Expr>,
    },
    Function {
        name: String,
        rettype: String,
        params: HashMap<String, String>,
        body: Box<Expr>,
    },
    Block(Vec<Expr>),
}
