#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Assign(String, Expr),
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
    Binary(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    PipeArrow,
    Plus,
    Minus,
    Multiply,
    Divide,
}