#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Div,
    Mult,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(i128),
    Float(f64),
    String(String),
    VariableRef(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    BinOp {
        op: BinOp,
        lhand: Box<Expr>,
        rhand: Box<Expr>,
    },
    Value(Value),
}
