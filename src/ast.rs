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

    Ternary {
        condition: Box<Expr>,
        true_branch: Box<Expr>,
        false_branch: Box<Expr>,
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

    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },

    ArrayAppend {
        array: Box<Expr>,
        value: Box<Expr>,
        index: Option<Box<Expr>>, // None = push to end, Some = insert at index
    },

    ArrayRemove {
        array: Box<Expr>,
        index: Option<Box<Expr>>, // None = last element, Some = specific index
        return_val: bool,          // false = return modified array, true = return removed element
    },

    Tuple(Vec<Expr>),

    Set(Vec<Expr>),

    Map(Vec<(Expr, Expr)>),

    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        step: Box<Expr>,
    },

    Filter {
        array: Box<Expr>,
        body: Box<Expr>,
    },

    MapExpr {
        array: Box<Expr>,
        body: Box<Expr>,
    },
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
    Scale,
    Descale,
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
    Round,
    ShiftLeft,
    ShiftRight,
    Sort,
    RevSort,
    Min,
    Max,
    Avg,
}