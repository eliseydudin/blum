use std::collections::HashMap;

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Div,
    Mult,
}

#[derive(Debug)]
pub enum Value {
    Integer(i128),
    Float(f64),
    String(String),
    VariableRef(String),
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
    Value(Value),
    Null,
}

impl Expr {
    pub fn variable_ref(var: String) -> Self {
        Expr::Value(Value::VariableRef(var))
    }
}
