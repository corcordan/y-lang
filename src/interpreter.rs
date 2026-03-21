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
                self.evaluate(expr);
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
                // Extract variable name for compound assignment before left is moved
                let assign_name = match op {
                    crate::ast::Operator::Increment | crate::ast::Operator::Decrement
                    | crate::ast::Operator::Scale | crate::ast::Operator::Descale
                    | crate::ast::Operator::ShiftLeft | crate::ast::Operator::ShiftRight => {
                        if let Expr::Identifier(ref n) = *left { Some(n.clone()) }
                        else { panic!("Cannot apply compound assignment to a non-variable") }
                    }
                    _ => None,
                };
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
                    crate::ast::Operator::Increment => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot increment non-number"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Increment amount must be a number"));
                        let new_val = (left_num + right_num).to_string();
                        if let Some(n) = assign_name { self.variables.insert(n, new_val.clone()); }
                        new_val
                    }
                    crate::ast::Operator::Decrement => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot decrement non-number"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Decrement amount must be a number"));
                        let new_val = (left_num - right_num).to_string();
                        if let Some(n) = assign_name { self.variables.insert(n, new_val.clone()); }
                        new_val
                    }
                    crate::ast::Operator::Scale => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot scale non-number"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Scale amount must be a number"));
                        let new_val = (left_num * right_num).to_string();
                        if let Some(n) = assign_name { self.variables.insert(n, new_val.clone()); }
                        new_val
                    }
                    crate::ast::Operator::Descale => {
                        let left_num: f64 = left_val.parse().unwrap_or_else(|_| panic!("Cannot descale non-number"));
                        let right_num: f64 = right_val.parse().unwrap_or_else(|_| panic!("Descale amount must be a number"));
                        if right_num == 0.0 { panic!("Descale by zero"); }
                        let new_val = (left_num / right_num).to_string();
                        if let Some(n) = assign_name { self.variables.insert(n, new_val.clone()); }
                        new_val
                    }
                    crate::ast::Operator::ShiftLeft => {
                        let mut elements = parse_array_string(&left_val);
                        let n = right_val.parse::<f64>().unwrap_or_else(|_| panic!("Shift amount must be a number")) as usize;
                        if !elements.is_empty() {
                            let n = n % elements.len();
                            elements.rotate_left(n);
                        }
                        let new_arr = format!("[{}]", elements.join(", "));
                        if let Some(name) = assign_name { self.variables.insert(name, new_arr.clone()); }
                        new_arr
                    }
                    crate::ast::Operator::ShiftRight => {
                        let mut elements = parse_array_string(&left_val);
                        let n = right_val.parse::<f64>().unwrap_or_else(|_| panic!("Shift amount must be a number")) as usize;
                        if !elements.is_empty() {
                            let n = n % elements.len();
                            elements.rotate_right(n);
                        }
                        let new_arr = format!("[{}]", elements.join(", "));
                        if let Some(name) = assign_name { self.variables.insert(name, new_arr.clone()); }
                        new_arr
                    }
                    _ => panic!("Operator not implemented: {:?}", op),
                }
            }
            Expr::UnaryPre { op, expr } => {
                let val = self.evaluate(*expr);
                match op {
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
                match op {
                    crate::ast::Operator::Increment => {
                        let name = if let Expr::Identifier(ref n) = *expr { Some(n.clone()) } else { None };
                        let val = self.evaluate(*expr);
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot increment non-number"));
                        let new_val = (num + 1.0).to_string();
                        if let Some(n) = name { self.variables.insert(n, new_val.clone()); }
                        new_val
                    }
                    crate::ast::Operator::Decrement => {
                        let name = if let Expr::Identifier(ref n) = *expr { Some(n.clone()) } else { None };
                        let val = self.evaluate(*expr);
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot decrement non-number"));
                        let new_val = (num - 1.0).to_string();
                        if let Some(n) = name { self.variables.insert(n, new_val.clone()); }
                        new_val
                    }
                    crate::ast::Operator::Factorial => {
                        let val = self.evaluate(*expr);
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
                        // support length for strings and arrays; numbers are invalid
                        match *expr {
                            Expr::String(ref s) => s.len().to_string(),
                            Expr::Array(ref arr) => arr.len().to_string(),
                            _ => {
                                let v = self.evaluate(*expr);
                                if v.parse::<f64>().is_ok() {
                                    panic!("Cannot take length of a number");
                                }
                                v.len().to_string()
                            }
                        }
                    }
                    crate::ast::Operator::Floor => {
                        let val = self.evaluate(*expr);
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot apply floor to non-number"));
                        num.floor().to_string()
                    }
                    crate::ast::Operator::Ceiling => {
                        let val = self.evaluate(*expr);
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot apply ceiling to non-number"));
                        num.ceil().to_string()
                    }
                    crate::ast::Operator::Modulo => {
                        let val = self.evaluate(*expr);
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot apply modulo to non-number"));
                        (num % 2.0).to_string()
                    }
                    crate::ast::Operator::Power => {
                        let val = self.evaluate(*expr);
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot apply power to non-number"));
                        (num.powf(2.0)).to_string()
                    }
                    _ => panic!("UnaryPost operator not implemented: {:?}", op),
                }
            }   
            Expr::Ternary { condition, true_branch, false_branch } => {
                let cond_val = self.evaluate(*condition);
                if cond_val != "0" && cond_val != "" {
                    self.evaluate(*true_branch)
                } else {
                    self.evaluate(*false_branch)
                }
            }
            Expr::Assign { name, value } => {
                let val = self.evaluate(*value);
                self.variables.insert(name.clone(), val.clone());
                val
            }
            Expr::Lambda { .. } => panic!("Lambda not implemented"),
            Expr::Array(elements) => {
                let values: Vec<String> = elements.into_iter().map(|e| self.evaluate(e)).collect();
                format!("[{}]", values.join(", "))
            }
            Expr::ArrayAppend { array, value, index } => {
                let name = if let Expr::Identifier(ref n) = *array { n.clone() }
                    else { panic!("Cannot append to a non-variable") };
                let arr_str = self.evaluate(*array);
                let val_str = self.evaluate(*value);
                let mut elements = parse_array_string(&arr_str);
                match index {
                    None => elements.push(val_str),
                    Some(idx_expr) => {
                        let idx: i64 = self.evaluate(*idx_expr).parse::<f64>().unwrap() as i64;
                        let len = elements.len() as i64;
                        let actual = (if idx < 0 { len + idx + 1 } else { idx }).clamp(0, len) as usize;
                        elements.insert(actual, val_str);
                    }
                }
                let new_arr = format!("[{}]", elements.join(", "));
                self.variables.insert(name, new_arr.clone());
                new_arr
            }
            Expr::ArrayRemove { array, index, return_val } => {
                let name = if let Expr::Identifier(ref n) = *array { n.clone() }
                    else { panic!("Cannot remove from a non-variable") };
                let arr_str = self.evaluate(*array);
                let mut elements = parse_array_string(&arr_str);
                let actual = match index {
                    None => elements.len().checked_sub(1).unwrap_or_else(|| panic!("Cannot remove from empty array")),
                    Some(idx_expr) => {
                        let idx: i64 = self.evaluate(*idx_expr).parse::<f64>().unwrap() as i64;
                        let len = elements.len() as i64;
                        let a = if idx < 0 { len + idx } else { idx };
                        if a < 0 || a >= len { panic!("Remove index {idx} out of bounds"); }
                        a as usize
                    }
                };
                let removed = elements.remove(actual);
                let new_arr = format!("[{}]", elements.join(", "));
                self.variables.insert(name, new_arr.clone());
                if return_val { removed } else { new_arr }
            }
            Expr::Index { array, index } => {
                let arr_val = self.evaluate(*array);
                let idx_val = self.evaluate(*index);
                let idx: i64 = idx_val.parse::<f64>()
                    .unwrap_or_else(|_| panic!("Array index must be a number")) as i64;
                let elements = parse_array_string(&arr_val);
                let len = elements.len() as i64;
                let actual = if idx < 0 { len + idx } else { idx };
                if actual < 0 || actual >= len {
                    panic!("Index {idx} out of bounds for array of length {len}");
                }
                elements[actual as usize].clone()
            }
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
                        let result = self.evaluate(args.into_iter().next().unwrap());
                        println!("{}", result);
                        result
                    }
                    _ => panic!("Unknown function: {}", name),
                }
            }
            // Non-identifier callee: evaluate it as an expression (args are ignored)
            // This allows `expr |> expr2` where expr2 is not a function name
            callee_expr => self.evaluate(callee_expr),
        }
    }
}

fn parse_array_string(s: &str) -> Vec<String> {
    let s = s.trim();
    if s == "[]" {
        return Vec::new();
    }
    let inner = &s[1..s.len() - 1]; // strip [ and ]
    inner.split(", ").map(|e| e.to_string()).collect()
}