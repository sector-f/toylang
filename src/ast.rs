use itertools::Itertools;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum Line {
    Statement(Statement),
    Expression(Expr),
}

#[derive(Debug)]
pub enum Statement {
    DeclareVar(String),
    AssignVar(String, Expr),
    ShadowVar(String, Expr),
    Print(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Literal(Value),
    Reference(String),
    Op(Op, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
}

#[derive(Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let text = match *self {
            Value::Int(num) => format!("{}", num),
            Value::Float(num) => format!("{}", num),
            Value::String(ref string) => format!("{}", string),
            Value::Boolean(b) => format!("{}", b),
            Value::Array(ref vec) => {
                let list = vec.iter().format_with(", ", |item, f| f(&format_args!("{}", item)));
                format!("[{}]", list)
            },
        };

        write!(f, "{}", text)
    }
}
