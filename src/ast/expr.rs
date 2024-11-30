use std::collections::HashMap;

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
    Function {
        name: String,
        rettype: String,
        params: HashMap<String, String>,
        body: Box<Expr>,
    },
    Block(Vec<Expr>),
    Value(Value),
    Null,
    Not(Box<Expr>),
}

impl Expr {
    pub fn variable_ref(var: String) -> Self {
        Expr::Value(Value::VariableRef(var))
    }
}
