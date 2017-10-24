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
