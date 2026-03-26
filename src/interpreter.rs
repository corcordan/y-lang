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
                                if v.starts_with('[') || v.starts_with('(') {
                                    parse_array_string(&v).len().to_string()
                                }
                                else {
                                    v.len().to_string()
                                }
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
                    crate::ast::Operator::Round => {
                        let val = self.evaluate(*expr);
                        let num: f64 = val.parse().unwrap_or_else(|_| panic!("Cannot apply round to non-number"));
                        num.round().to_string()
                    }
                    crate::ast::Operator::Min => {
                        let v = self.evaluate(*expr);
                        if v.starts_with('[') {
                            let elements = parse_array_string(&v);
                            elements.into_iter().min_by(|a, b| {
                                match (a.parse::<f64>(), b.parse::<f64>()) {
                                    (Ok(a), Ok(b)) => a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal),
                                    _ => a.cmp(b),
                                }
                            }).unwrap_or_else(|| panic!("Cannot get min of empty array"))
                        } else {
                            panic!("Can only get the min of an array");
                        }
                    }
                    crate::ast::Operator::Max => {
                        let v = self.evaluate(*expr);
                        if v.starts_with('[') {
                            let elements = parse_array_string(&v);
                            elements.into_iter().max_by(|a, b| {
                                match (a.parse::<f64>(), b.parse::<f64>()) {
                                    (Ok(a), Ok(b)) => a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal),
                                    _ => a.cmp(b),
                                }
                            }).unwrap_or_else(|| panic!("Cannot get max of empty array"))
                        } else {
                            panic!("Can only get the max of an array");
                        }
                    }
                    crate::ast::Operator::Avg => {
                        let v = self.evaluate(*expr);
                        if v.starts_with('[') {
                            let elements = parse_array_string(&v);
                            if elements.is_empty() { panic!("Cannot get avg of empty array"); }
                            let sum: f64 = elements.iter()
                                .map(|e| e.parse::<f64>().unwrap_or_else(|_| panic!("Avg requires numeric array")))
                                .sum();
                            (sum / elements.len() as f64).to_string()
                        } else {
                            panic!("Can only get the avg of an array");
                        }
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
                    crate::ast::Operator::Sort => {
                        let v = self.evaluate(*expr);
                        if v.parse::<f64>().is_ok() {
                            panic!("Cannot sort a number");
                        }
                        if v.starts_with('(') {
                            panic!("Cannot modify a tuple");
                        }
                        if v.starts_with('{') {
                            panic!("Cannot sort a set or hashmap");
                        }
                        if v.starts_with('[') {
                            let mut elements = parse_array_string(&v);
                            elements.sort_by(|a, b| {
                                match (a.parse::<f64>(), b.parse::<f64>()) {
                                    (Ok(a), Ok(b)) => a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal),
                                    _ => a.cmp(b),
                                }
                            });
                            format!("[{}]", elements.join(", "))
                        } else {
                            let mut chars: Vec<char> = v.chars().collect();
                            chars.sort();
                            chars.into_iter().collect()
                        }
                    }
                    crate::ast::Operator::RevSort => {
                        let v = self.evaluate(*expr);
                        if v.parse::<f64>().is_ok() {
                            panic!("Cannot sort a number");
                        }
                        if v.starts_with('[') {
                            let mut elements = parse_array_string(&v);
                            elements.sort_by(|a, b| {
                                match (a.parse::<f64>(), b.parse::<f64>()) {
                                    (Ok(a), Ok(b)) => b.partial_cmp(&a).unwrap_or(std::cmp::Ordering::Equal),
                                    _ => b.cmp(a),
                                }
                            });
                            format!("[{}]", elements.join(", "))
                        } else {
                            let mut chars: Vec<char> = v.chars().collect();
                            chars.sort_by(|a, b| b.cmp(a));
                            chars.into_iter().collect()
                        }
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

                if arr_str.starts_with('(') {
                    panic!("Cannot modify a tuple");
                }

                if arr_str.starts_with('[') {
                    // Array: positional insert
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
                } else {
                    // Set or Hashmap: hashmap if index present or value is a Map node
                    let is_map_op = index.is_some() || matches!(*value, Expr::Map(_));
                    if is_map_op {
                        // Hashmap: add/update key-value pair
                        let (key_str, val_str) = if let Expr::Map(mut pairs) = *value {
                            if pairs.len() != 1 { panic!("Map entry must have exactly one key-value pair"); }
                            let (k, v) = pairs.remove(0);
                            (self.evaluate(k), self.evaluate(v))
                        } else {
                            let k = self.evaluate(*value);
                            let v = self.evaluate(*index.expect("Expected value after ':'"));
                            (k, v)
                        };
                        let mut entries = parse_map_string(&arr_str);
                        if let Some(e) = entries.iter_mut().find(|(k, _)| k == &key_str) {
                            e.1 = val_str;
                        } else {
                            entries.push((key_str, val_str));
                        }
                        let new_map = format!("{{{}}}", entries.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join(", "));
                        self.variables.insert(name, new_map.clone());
                        new_map
                    } else {
                        // Set: add only if not already present
                        let val_str = self.evaluate(*value);
                        let mut elements = parse_set_string(&arr_str);
                        if !elements.contains(&val_str) {
                            elements.push(val_str);
                        }
                        let new_set = format!("{{{}}}", elements.join(", "));
                        self.variables.insert(name, new_set.clone());
                        new_set
                    }
                }
            }
            Expr::ArrayRemove { array, index, return_val } => {
                let name = if let Expr::Identifier(ref n) = *array { n.clone() }
                    else { panic!("Cannot remove from a non-variable") };
                let arr_str = self.evaluate(*array);

                if arr_str.starts_with('(') {
                    panic!("Cannot modify a tuple");
                }

                if arr_str.starts_with('[') {
                    // Array: positional remove
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
                } else {
                    // Set or Hashmap: remove by value/key
                    let target = match index {
                        None => panic!("Must specify a value to remove from a set or hashmap"),
                        Some(idx_expr) => self.evaluate(*idx_expr),
                    };
                    if is_hashmap_string(&arr_str) {
                        // Hashmap: remove by key
                        let mut entries = parse_map_string(&arr_str);
                        let pos = entries.iter().position(|(k, _)| k == &target)
                            .unwrap_or_else(|| panic!("Key '{}' not found in hashmap", target));
                        let (_, removed_val) = entries.remove(pos);
                        let new_map = if entries.is_empty() { "{}".to_string() } else {
                            format!("{{{}}}", entries.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join(", "))
                        };
                        self.variables.insert(name, new_map.clone());
                        if return_val { removed_val } else { new_map }
                    } else {
                        // Set: remove by value
                        let mut elements = parse_set_string(&arr_str);
                        let pos = elements.iter().position(|e| e == &target)
                            .unwrap_or_else(|| panic!("Value '{}' not found in set", target));
                        let removed = elements.remove(pos);
                        let new_set = if elements.is_empty() { "{}".to_string() } else {
                            format!("{{{}}}", elements.join(", "))
                        };
                        self.variables.insert(name, new_set.clone());
                        if return_val { removed } else { new_set }
                    }
                }
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
            Expr::Tuple(elements) => {
                let vals: Vec<String> = elements.into_iter().map(|e| self.evaluate(e)).collect();
                format!("({})", vals.join(", "))
            }
            Expr::Set(elements) => {
                let mut seen = std::collections::HashSet::new();
                let mut vals: Vec<String> = Vec::new();
                for e in elements {
                    let v = self.evaluate(e);
                    if seen.insert(v.clone()) {
                        vals.push(v);
                    }
                }
                format!("{{{}}}", vals.join(", "))
            }
            Expr::Map(pairs) => {
                let entries: Vec<String> = pairs.into_iter()
                    .map(|(k, v)| format!("{}: {}", self.evaluate(k), self.evaluate(v)))
                    .collect();
                format!("{{{}}}", entries.join(", "))
            }
            Expr::Range { start, end, step } => {
                let start_val: f64 = self.evaluate(*start).parse().unwrap_or_else(|_| panic!("Range start must be a number"));
                let end_val: f64 = self.evaluate(*end).parse().unwrap_or_else(|_| panic!("Range end must be a number"));
                let step_val: f64 = self.evaluate(*step).parse().unwrap_or_else(|_| panic!("Range step must be a number"));
                if step_val == 0.0 { panic!("Range step cannot be 0"); }
                let mut values = Vec::new();
                let mut current = start_val;
                if step_val > 0.0 {
                    while current < end_val { values.push(current.to_string()); current += step_val; }
                } else {
                    while current > end_val { values.push(current.to_string()); current += step_val; }
                }
                format!("[{}]", values.join(", "))
            }
            Expr::Filter { array, body } => {
                let arr_str = self.evaluate(*array);
                let elements = parse_array_string(&arr_str);
                let mut result = Vec::new();
                for elem in elements {
                    self.variables.insert("_".to_string(), elem.clone());
                    let val = self.evaluate(*body.clone());
                    if is_truthy(&val) {
                        result.push(elem);
                    }
                }
                format!("[{}]", result.join(", "))
            }
            Expr::MapExpr { array, body } => {
                let arr_str = self.evaluate(*array);
                let elements = parse_array_string(&arr_str);
                let mut result = Vec::new();
                for elem in elements {
                    self.variables.insert("_".to_string(), elem);
                    result.push(self.evaluate(*body.clone()));
                }
                format!("[{}]", result.join(", "))
            }
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

// Split a comma-separated list respecting nested brackets/parens/braces
fn split_collection(inner: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    let mut chars = inner.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '[' | '{' | '(' => { depth += 1; current.push(ch); }
            ']' | '}' | ')' => { depth -= 1; current.push(ch); }
            ',' if depth == 0 => {
                if chars.peek() == Some(&' ') { chars.next(); }
                result.push(current.trim().to_string());
                current = String::new();
            }
            _ => { current.push(ch); }
        }
    }
    if !current.trim().is_empty() {
        result.push(current.trim().to_string());
    }
    result
}

fn parse_array_string(s: &str) -> Vec<String> {
    let s = s.trim();
    if s == "[]" { return Vec::new(); }
    split_collection(&s[1..s.len() - 1])
}

fn parse_set_string(s: &str) -> Vec<String> {
    let s = s.trim();
    if s == "{}" { return Vec::new(); }
    split_collection(&s[1..s.len() - 1])
}

fn parse_map_string(s: &str) -> Vec<(String, String)> {
    let s = s.trim();
    if s == "{}" { return Vec::new(); }
    split_collection(&s[1..s.len() - 1]).into_iter().map(|entry| {
        let colon = entry.find(": ").expect("Invalid map entry");
        (entry[..colon].to_string(), entry[colon + 2..].to_string())
    }).collect()
}

fn is_hashmap_string(s: &str) -> bool {
    let s = s.trim();
    if s == "{}" { return false; }
    if !s.starts_with('{') { return false; }
    let inner = &s[1..s.len() - 1];
    inner.contains(": ")
}

fn is_truthy(s: &str) -> bool {
    if let Ok(n) = s.parse::<f64>() {
        n != 0.0
    } else {
        s != "[]" && s != "{}" && s != "()" && !s.is_empty()
    }
}