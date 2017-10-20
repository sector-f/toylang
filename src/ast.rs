pub enum AstNode {
    VarDeclaration(String),
    VarAssign(String, Expression),
}

#[derive(Debug)]
pub enum Expression {
    Num(f64),
    Str(String),
    Bool(bool),
    Array(Vec<Expression>),
}
