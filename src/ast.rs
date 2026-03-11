pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Expr,
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Identifier(String),

    Assign {
        name: String,
        value: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },

    UnaryPre {
        op: Operator,
        expr: Box<Expr>,
    },

    UnaryPost {
        op: Operator,
        expr: Box<Expr>,
    },

    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },

    Array(Vec<Expr>),

    Map(Vec<(Expr, Expr)>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Increment,
    Decrement,
    Negate,
    Not,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Equal,
    NotEqual,

    And,
    Or,
    Xor,
    Nand,
    Nor,
    Xnor,

    Factorial,
    Length,
    Floor,
    Ceiling,
}