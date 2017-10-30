use itertools::Itertools;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq)]
pub enum Line {
    Statement(Statement),
    Expression(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    DeclareVar(String, Expr),
    MutateVar(AssignOp, String, Expr),
    Expression(Expr),
    If(IfStatement, Option<Vec<IfStatement>>, Option<Vec<Statement>>), // (If, Else If, Else)
    While(Expr, Vec<Statement>),
    Print(Vec<Expr>),
    Println(Vec<Expr>),
    Typeof(Box<Expr>),
    Exit(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub e: Expr,
    pub s: Vec<Statement>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Reference(String),
    Typecast(Box<Expr>, Box<Expr>),
    Array(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    BinOp(Op, Box<Expr>, Box<Expr>),
    Comparison(CompOp, Box<Expr>, Box<Expr>),
    BoolChain(BoolLogic, Box<Expr>, Box<Expr>),
    UnOp(UnaryOp, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignOp {
    Equals,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompOp {
    Equal,
    NotEq,
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoolLogic {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Num,
    String,
    Boolean,
    Array,
    Type,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let text = match *self {
            Type::Num => "num",
            Type::String => "string",
            Type::Boolean => "bool",
            Type::Array => "array",
            Type::Type => "type",
        };

        write!(f, "{}", text)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Num(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Type(Type),
}

impl Value {
    pub fn get_type(&self) -> Type {
        match *self {
            Value::Num(_) => Type::Num,
            Value::String(_) => Type::String,
            Value::Boolean(_) => Type::Boolean,
            Value::Array(_) => Type::Array,
            Value::Type(_) => Type::Type,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let text = match *self {
            Value::Num(num) => num.to_string(),
            Value::String(ref string) => string.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Array(ref vec) => {
                let list = vec.iter().format_with(", ", |item, f| f(&format_args!("{}", item)));
                format!("[{}]", list)
            },
            Value::Type(ref t) => t.to_string(),
        };

        write!(f, "{}", text)
    }
}
