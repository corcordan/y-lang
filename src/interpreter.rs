use crate::ast::{Stmt, Expr, Operator};

pub struct Interpreter {
    variables: std::collections::HashMap<String, String>
}

impl Interpreter {
    // Create a new interpreter instance
    pub fn new() -> Self {
        Interpreter {
            variables: std::collections::HashMap::new(),
        }
    }

    // Interpret a vector of statements
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.execute_statement(stmt);
        }
    }

    // Execute a single statement
    fn execute_statement(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                let result = self.evaluate(expr);
                println!("{}", result);
            }
            Stmt::FunctionDeclaration { .. } => {
                // Skip function declarations for now
            }
        }
    }

    // Evaluate an expression and return its value as a string
    fn evaluate(&mut self, expr: Expr) -> String {
        match expr {
            Expr::Identifier(name) => {
                // Check for builtin functions
                match name.as_str() {
                    "p" => panic!("'p' should be called as a function, not referenced directly"),
                    _ => self.variables.get(&name).cloned().unwrap_or_else(|| panic!("Undefined variable: {}", name)),
                }
            }
            Expr::Number(num) => num.to_string(),
            Expr::String(s) => s,
            Expr::Call { callee, args } => self.evaluate_call(*callee, args),
            Expr::Binary { .. } => panic!("Binary expressions not implemented"),
            Expr::Assign { .. } => panic!("Assignment not implemented"),
            Expr::Lambda { .. } => panic!("Lambda not implemented"),
            Expr::Array(_) => panic!("Array not implemented"),
            Expr::Map(_) => panic!("Map not implemented"),
        }
    }

    fn evaluate_call(&mut self, callee: Expr, args: Vec<Expr>) -> String {
        match callee {
            Expr::Identifier(name) => {
                match name.as_str() {
                    "p" => {
                        // Print function - expects exactly one argument
                        if args.len() != 1 {
                            panic!("'p' function expects exactly one argument");
                        }
                        self.evaluate(args.into_iter().next().unwrap())
                    }
                    _ => panic!("Unknown function: {}", name),
                }
            }
            _ => panic!("Can only call functions by name"),
        }
    }
}