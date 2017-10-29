use itertools::Itertools;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum Line {
    Statement(Statement),
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub enum Statement {
    DeclareVar(String, Expr),
    MutateVar(AssignOp, String, Expr),
    Expression(Expr),
    If(IfStatement, Option<Vec<IfStatement>>, Option<Vec<Statement>>), // (If, Else If, Else)
    While(Expr, Vec<Statement>),
    Print(Expr),
    Exit(Expr),
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub e: Expr,
    pub s: Vec<Statement>
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Reference(String),
    Index(Box<Expr>, Box<Expr>),
    BinOp(Op, Box<Expr>, Box<Expr>),
    Comparison(CompOp, Box<Expr>, Box<Expr>),
    BoolChain(BoolLogic, Box<Expr>, Box<Expr>),
    UnOp(UnaryOp, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
}

#[derive(Debug, Clone)]
pub enum AssignOp {
    Equals,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
}

#[derive(Debug, Clone)]
pub enum CompOp {
    Equal,
    NotEq,
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Debug, Clone)]
pub enum BoolLogic {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
}

#[derive(Debug, Clone)]
pub enum Value {
    Num(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
}

impl Value {
    pub fn get_type(&self) -> &str {
        match *self {
            Value::Num(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Array(_) => "array",
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let text = match *self {
            Value::Num(num) => format!("{}", num),
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
