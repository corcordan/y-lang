use crate::ast::{Stmt, Expr};

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
            Expr::Binary { left, op, right } => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                
                match op {
                    crate::ast::Operator::Plus => {
                        // For now, assume numbers
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot add non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot add non-numbers"));
                        (left_num + right_num).to_string()
                    }
                    crate::ast::Operator::Minus => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot subtract non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot subtract non-numbers"));
                        (left_num - right_num).to_string()
                    }
                    crate::ast::Operator::Multiply => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot multiply non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot multiply non-numbers"));
                        (left_num * right_num).to_string()
                    }
                    crate::ast::Operator::Divide => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot divide non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot divide non-numbers"));
                        if right_num == 0.0 {
                            panic!("Division by zero");
                        }
                        (left_num / right_num).to_string()
                    }
                    crate::ast::Operator::Modulo => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot modulo non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot modulo non-numbers"));
                        if right_num == 0.0 {
                            panic!("Modulo by zero");
                        }
                        (left_num % right_num).to_string()
                    }
                    crate::ast::Operator::Power => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot exponentiate non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot exponentiate non-numbers"));
                        left_num.powf(right_num).to_string()
                    }
                    crate::ast::Operator::Greater => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot compare non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot compare non-numbers"));
                        if left_num > right_num { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::GreaterEqual => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot compare non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot compare non-numbers"));
                        if left_num >= right_num { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::Less => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot compare non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot compare non-numbers"));
                        if left_num < right_num { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::LessEqual => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot compare non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot compare non-numbers"));
                        if left_num <= right_num { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::Equal => {
                        if left_val == right_val { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::NotEqual => {
                        if left_val != right_val { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::And => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'and' to non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'and' to non-numbers"));
                        if left_num != 0.0 && right_num != 0.0 { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::Or => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'or' to non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'or' to non-numbers"));
                        if left_num != 0.0 || right_num != 0.0 { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::Xor => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'xor' to non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'xor' to non-numbers"));
                        if (left_num != 0.0) ^ (right_num != 0.0) { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::Nand => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'nand' to non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'nand' to non-numbers"));
                        if !(left_num != 0.0 && right_num != 0.0) { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::Nor => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'nor' to non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'nor' to non-numbers"));
                        if !(left_num != 0.0 || right_num != 0.0) { "1".to_string() } else { "0".to_string() }
                    }
                    crate::ast::Operator::Xnor => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'xnor' to non-numbers"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Cannot apply 'xnor' to non-numbers"));
                        if !((left_num != 0.0) ^ (right_num != 0.0)) { "1".to_string() } else { "0".to_string() }
                    }
                    _ => panic!("Operator not implemented: {:?}", op),
                }
            }
            Expr::UnaryPre { op, expr } => {
                let val = self.evaluate(*expr);
                match op {
                    crate::ast::Operator::Increment => {
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot increment non-number"));
                        (num + 1.0).to_string()
                    }
                    crate::ast::Operator::Decrement => {
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot decrement non-number"));
                        (num - 1.0).to_string()
                    }
                    crate::ast::Operator::Negate => {
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot negate non-number"));
                        (-num).to_string()
                    }
                    crate::ast::Operator::Not => {
                        // For now, treat as numeric not (0 -> 1, non-zero -> 0)
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot apply not to non-number"));
                        if num == 0.0 { "1".to_string() } else { "0".to_string() }
                    }
                    _ => panic!("UnaryPre operator not implemented: {:?}", op),
                }
            }
            Expr::UnaryPost { op, expr } => {
                let val = self.evaluate(*expr);
                match op {
                    crate::ast::Operator::Increment => {
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot increment non-number"));
                        (num + 1.0).to_string()
                    }
                    crate::ast::Operator::Decrement => {
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot decrement non-number"));
                        (num - 1.0).to_string()
                    }
                    crate::ast::Operator::Factorial => {
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot apply factorial to non-number"));
                        if num < 0.0 {
                            panic!("Cannot apply factorial to negative number");
                        }
                        let mut result = 1.0;
                        for i in 1..=num as u64 {
                            result *= i as f64;
                        }
                        result.to_string()
                    }
                    crate::ast::Operator::Length => {
                        // For now, only support length of strings
                        val.len().to_string()
                    }
                    crate::ast::Operator::Floor => {
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot apply floor to non-number"));
                        num.floor().to_string()
                    }
                    crate::ast::Operator::Ceiling => {
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot apply ceiling to non-number"));
                        num.ceil().to_string()
                    }
                    _ => panic!("UnaryPost operator not implemented: {:?}", op),
                }
            }   
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