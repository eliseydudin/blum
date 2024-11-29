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
}
