use crate::lexer::{Lexer, Token};
use crate::ast::{Stmt, Expr};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    // Create a new parser from the lexer
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
            peek_token: Token::EOF,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    // Advance to the next token
    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    // Parse the entire program and return a vector of statements
    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        self.skip_newlines();
        while self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.skip_newlines();
        }
        statements
    }

    fn skip_newlines(&mut self) {
        while self.current_token == Token::Newline {
            self.next_token();
        }
    }

    // Parse a single statement
    fn parse_statement(&mut self) -> Option<Stmt> {
        self.parse_expression().map(Stmt::Expression)
    }

    fn parse_expression(&mut self) -> Option<Expr> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Option<Expr> {
        let expr = self.parse_pipe()?;

        if let Token::Assign = self.current_token {
            if let Expr::Identifier(name) = expr {
                self.next_token();
                let value = self.parse_assignment()?;
                return Some(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else {
                panic!("Invalid assignment target");
            }
        }

        Some(expr)
    }

    // Parse compound assignment: expr += expr, expr -= expr, expr *= expr, or expr /= expr
    // Could conflict with assignment since higher precedence
    fn parse_compound_assign(&mut self) -> Option<Expr> {
        let expr = self.parse_ternary()?;

        if matches!(self.current_token, Token::Increment | Token::Decrement | Token::Scale | Token::Descale) {
            let op = if self.current_token == Token::Increment {
                crate::ast::Operator::Increment
            } 
            else if self.current_token == Token::Decrement {
                crate::ast::Operator::Decrement
            }
            else if self.current_token == Token::Scale {
                crate::ast::Operator::Scale
            } 
            else {
                crate::ast::Operator::Descale
            };
            let default_right = if matches!(op, crate::ast::Operator::Scale | crate::ast::Operator::Descale) {
                Expr::Number(2.0)
            } else {
                Expr::Number(1.0)
            };
            self.next_token(); // consume += or -= or *= or /=
            let right = match self.current_token {
                Token::Number(_) | Token::Identifier(_) | Token::LParen
                | Token::Minus | Token::Plus => self.parse_pipe()?,
                _ => default_right,
            };
            return Some(Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            });
        }

        Some(expr)
    }

    // Parse pipe operations: expr |> expr
    // Pipe has lower precedence than compound assign so (x += expr) |> f works without parens
    // Arithmetic operators after |> are treated as operator sections:
    //   expr |> + n    =>  expr + n
    //   expr |> /      =>  length(expr)   (no right operand)
    //   expr |> / n    =>  expr / n       (right operand present)
    fn parse_range(&mut self) -> Option<Expr> {
        // Bare .. with no start value
        if self.current_token == Token::Range {
            self.next_token(); // consume ..
            let end = if let Token::Number(n) = self.current_token {
                let n = n; self.next_token(); Expr::Number(n)
            } else {
                Expr::Number(10.0) // default end
            };
            let step = if self.current_token == Token::Range {
                self.next_token();
                if let Token::Number(n) = self.current_token { let n = n; self.next_token(); Expr::Number(n) }
                else { Expr::Number(1.0) }
            } else { Expr::Number(1.0) };
            return Some(Expr::Range { start: Box::new(Expr::Number(0.0)), end: Box::new(end), step: Box::new(step) });
        }

        let expr = self.parse_compound_assign()?;

        if self.current_token == Token::Range {
            self.next_token(); // consume ..
            let end = self.parse_compound_assign()?;
            let step = if self.current_token == Token::Range {
                self.next_token();
                self.parse_compound_assign()?
            } else {
                Expr::Number(1.0)
            };
            Some(Expr::Range { start: Box::new(expr), end: Box::new(end), step: Box::new(step) })
        } else {
            Some(expr)
        }
    }

    fn parse_pipe(&mut self) -> Option<Expr> {
        let mut expr = self.parse_range()?;

        while self.current_token == Token::PipeArrow {
            self.next_token(); // consume '|>'

            // Check if the right side begins with an operator token that should be
            // applied to the piped value as the left operand
            let next_starts_expr = matches!(
                self.peek_token,
                Token::Number(_) | Token::String(_) | Token::Identifier(_)
                | Token::LParen | Token::Minus | Token::Bang | Token::Plus
            );

            expr = match self.current_token {
                Token::Plus => {
                    self.next_token();
                    let right = self.parse_compound_assign()?;
                    Expr::Binary { left: Box::new(expr), op: crate::ast::Operator::Plus, right: Box::new(right) }
                }
                Token::Minus => {
                    self.next_token();
                    let right = self.parse_compound_assign()?;
                    Expr::Binary { left: Box::new(expr), op: crate::ast::Operator::Minus, right: Box::new(right) }
                }
                Token::Star => {
                    self.next_token();
                    let right = self.parse_compound_assign()?;
                    Expr::Binary { left: Box::new(expr), op: crate::ast::Operator::Multiply, right: Box::new(right) }
                }
                Token::Slash => {
                    self.next_token();
                    if next_starts_expr {
                        let right = self.parse_compound_assign()?;
                        Expr::Binary { left: Box::new(expr), op: crate::ast::Operator::Divide, right: Box::new(right) }
                    } else {
                        Expr::UnaryPost { op: crate::ast::Operator::Length, expr: Box::new(expr) }
                    }
                }
                Token::Modulo => {
                    self.next_token();
                    if next_starts_expr {
                        let right = self.parse_compound_assign()?;
                        Expr::Binary { left: Box::new(expr), op: crate::ast::Operator::Modulo, right: Box::new(right) }
                    } else {
                        Expr::UnaryPost { op: crate::ast::Operator::Modulo, expr: Box::new(expr) }
                    }
                }
                Token::Power => {
                    self.next_token();
                    if next_starts_expr {
                        let right = self.parse_compound_assign()?;
                        Expr::Binary { left: Box::new(expr), op: crate::ast::Operator::Power, right: Box::new(right) }
                    } else {
                        Expr::UnaryPost { op: crate::ast::Operator::Power, expr: Box::new(expr) }
                    }
                }
                Token::Bang => {
                    self.next_token();
                    Expr::UnaryPost { op: crate::ast::Operator::Factorial, expr: Box::new(expr) }
                }
                Token::Underscore => {
                    self.next_token();
                    Expr::UnaryPost { op: crate::ast::Operator::Floor, expr: Box::new(expr) }
                }
                Token::Caret => {
                    self.next_token();
                    Expr::UnaryPost { op: crate::ast::Operator::Ceiling, expr: Box::new(expr) }
                }
                Token::Sort => {
                    self.next_token();
                    Expr::UnaryPost { op: crate::ast::Operator::Sort, expr: Box::new(expr) }
                }
                Token::RevSort => {
                    self.next_token();
                    Expr::UnaryPost { op: crate::ast::Operator::RevSort, expr: Box::new(expr) }
                }
                Token::Tilde => {
                    self.next_token();
                    Expr::UnaryPost { op: crate::ast::Operator::Round, expr: Box::new(expr) }
                }
                Token::Backslash => {
                    self.next_token();
                    Expr::UnaryPost { op: crate::ast::Operator::Avg, expr: Box::new(expr) }
                }
                Token::Less => {
                    self.next_token();
                    Expr::UnaryPost { op: crate::ast::Operator::Min, expr: Box::new(expr) }
                }
                Token::Greater => {
                    self.next_token();
                    Expr::UnaryPost { op: crate::ast::Operator::Max, expr: Box::new(expr) }
                }
                _ => {
                    // Check for f:body (filter) or m:body (map array)
                    let hof = if let Token::Identifier(ref n) = self.current_token {
                        if self.peek_token == Token::Colon { Some(n.clone()) } else { None }
                    } else { None };

                    if let Some(name) = hof {
                        if name == "f" || name == "m" {
                            self.next_token(); // consume 'f' or 'm'
                            self.next_token(); // consume ':'
                            let body = self.parse_compound_assign()?;
                            if name == "f" {
                                Expr::Filter { array: Box::new(expr), body: Box::new(body) }
                            } else {
                                Expr::MapExpr { array: Box::new(expr), body: Box::new(body) }
                            }
                        } else {
                            let right = self.parse_compound_assign()?;
                            Expr::Call { callee: Box::new(right), args: vec![expr] }
                        }
                    } else {
                        let right = self.parse_compound_assign()?;
                        Expr::Call { callee: Box::new(right), args: vec![expr] }
                    }
                }
            };
        }

        Some(expr)
    }

    fn parse_ternary(&mut self) -> Option<Expr> {
        let mut expr = self.parse_or()?;

        if self.current_token == Token::Question {
            self.next_token();
            let true_expr = self.parse_ternary()?;
            if self.current_token != Token::Colon {
                panic!("Expected ':' in ternary expression");
            }
            self.next_token();
            let false_expr = self.parse_ternary()?;
            expr = Expr::Ternary {
                condition: Box::new(expr),
                true_branch: Box::new(true_expr),
                false_branch: Box::new(false_expr),
            };
        }

        Some(expr)
    }

    fn parse_or(&mut self) -> Option<Expr> {
        let mut expr = self.parse_xor()?;

        while matches!(self.current_token, Token::Or | Token::Nor) {
            let op = match self.current_token {
                Token::Or => crate::ast::Operator::Or,
                Token::Nor => crate::ast::Operator::Nor,
                _ => unreachable!(),
            };
            self.next_token();
            let right = self.parse_xor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn parse_xor(&mut self) -> Option<Expr> {
        let mut expr = self.parse_and()?;

        while matches!(self.current_token, Token::Xor | Token::Xnor) {
            let op = match self.current_token {
                Token::Xor => crate::ast::Operator::Xor,
                Token::Xnor => crate::ast::Operator::Xnor,
                _ => unreachable!(),
            };
            self.next_token();
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn parse_and(&mut self) -> Option<Expr> {
        let mut expr = self.parse_equality()?;

        while matches!(self.current_token, Token::And | Token::Nand) {
            let op = match self.current_token {
                Token::And => crate::ast::Operator::And,
                Token::Nand => crate::ast::Operator::Nand,
                _ => unreachable!(),
            };
            self.next_token();
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn parse_equality(&mut self) -> Option<Expr> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.current_token, Token::Equal | Token::NotEqual) {
            let op = match self.current_token {
                Token::Equal => crate::ast::Operator::Equal,
                Token::NotEqual => crate::ast::Operator::NotEqual,
                _ => unreachable!(),
            };
            self.next_token();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut expr = self.parse_term()?;

        while matches!(self.current_token, Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual) {
            let op = match self.current_token {
                Token::Greater => crate::ast::Operator::Greater,
                Token::GreaterEqual => crate::ast::Operator::GreaterEqual,
                Token::Less => crate::ast::Operator::Less,
                Token::LessEqual => crate::ast::Operator::LessEqual,
                _ => unreachable!(),
            };
            self.next_token();
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_factor()?;

        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = match self.current_token {
                Token::Plus => crate::ast::Operator::Plus,
                Token::Minus => crate::ast::Operator::Minus,
                _ => unreachable!(),
            };
            self.next_token();
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut expr = self.parse_power()?;

        while matches!(self.current_token, Token::Star | Token::Slash | Token::DoubleSlash | Token::Modulo) {
            let op = match self.current_token {
                Token::Star => crate::ast::Operator::Multiply,
                Token::Slash => crate::ast::Operator::Divide,
                Token::DoubleSlash => crate::ast::Operator::Divide, // For now, treat '//' as '/'
                Token::Modulo => crate::ast::Operator::Modulo,
                _ => unreachable!(),
            };
            self.next_token();
            let right = self.parse_power()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_power(&mut self) -> Option<Expr> {
        let mut expr = self.parse_unary()?;

        while self.current_token == Token::Power {
            self.next_token();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: crate::ast::Operator::Power,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Minus => {
                self.next_token();
                let expr = self.parse_unary()?;
                Some(Expr::UnaryPre {
                    op: crate::ast::Operator::Negate,
                    expr: Box::new(expr),
                })
            }
            Token::Bang => {
                self.next_token();
                let expr = self.parse_unary()?;
                Some(Expr::UnaryPre {
                    op: crate::ast::Operator::Not,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        while matches!(self.current_token,
            Token::Modulo | 
            Token::Power | 
            Token::Bang | 
            Token::Slash | 
            Token::Underscore | 
            Token::Caret | 
            Token::At | 
            Token::Add | 
            Token::Remove | 
            Token::ShiftLeft | 
            Token::ShiftRight | 
            Token::Sort | 
            Token::RevSort |
            Token::Less |
            Token::Greater |
            Token::Backslash |
            Token::Tilde
        ) {
            // when encountering slash, power, modulo, or comparison tokens, ensure we aren't
            // looking at a binary operator (i.e. another expression follows on the right)
            if matches!(self.current_token, Token::Slash | Token::Power | Token::Modulo | Token::Less | Token::Greater) {
                match self.peek_token {
                    Token::Number(_)
                    | Token::String(_)
                    | Token::Identifier(_)
                    | Token::LParen
                    | Token::Minus
                    | Token::Bang
                    | Token::Plus
                    | Token::Underscore => break,
                    _ => {}
                }
            }

            if self.current_token == Token::At {
                self.next_token(); // consume @
                let index = match self.current_token {
                    Token::Number(n) => {
                        let n = n;
                        self.next_token();
                        Expr::Number(n)
                    }
                    Token::Minus => {
                        self.next_token(); // consume -
                        match self.current_token {
                            Token::Number(n) => {
                                let n = n;
                                self.next_token();
                                Expr::Number(-n)
                            }
                            _ => Expr::Number(-1.0), // bare @- means last element
                        }
                    }
                    _ => Expr::Number(0.0), // bare @ means first element
                };
                expr = Expr::Index {
                    array: Box::new(expr),
                    index: Box::new(index),
                };
                continue;
            }

            if matches!(self.current_token, Token::ShiftLeft | Token::ShiftRight) {
                let op = if self.current_token == Token::ShiftLeft {
                    crate::ast::Operator::ShiftLeft
                } else {
                    crate::ast::Operator::ShiftRight
                };
                self.next_token(); // consume << or >>
                let amount = match self.current_token {
                    Token::Number(n) => { let n = n; self.next_token(); Expr::Number(n) }
                    _ => Expr::Number(1.0),
                };
                expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(amount) };
                continue;
            }

            if self.current_token == Token::Add {
                self.next_token(); // consume ++
                let value = self.parse_unary()?;
                let index = if self.current_token == Token::Colon {
                    self.next_token(); // consume :
                    let idx = self.parse_unary()?;
                    Some(Box::new(idx))
                } else {
                    None
                };
                expr = Expr::ArrayAppend {
                    array: Box::new(expr),
                    value: Box::new(value),
                    index,
                };
                continue;
            }

            if self.current_token == Token::Remove {
                self.next_token(); // consume --
                let return_val = if self.current_token == Token::Bang {
                    self.next_token(); // consume !
                    true
                } else {
                    false
                };
                let index = match &self.current_token {
                    Token::Number(n) => {
                        let n = *n;
                        self.next_token();
                        Some(Box::new(Expr::Number(n)))
                    }
                    Token::Minus => {
                        self.next_token();
                        match self.current_token {
                            Token::Number(n) => {
                                let n = n;
                                self.next_token();
                                Some(Box::new(Expr::Number(-n)))
                            }
                            _ => Some(Box::new(Expr::Number(-1.0))),
                        }
                    }
                    Token::String(s) => {
                        let s = s.clone();
                        self.next_token();
                        Some(Box::new(Expr::String(s)))
                    }
                    Token::Identifier(name) => {
                        let name = name.clone();
                        self.next_token();
                        Some(Box::new(Expr::Identifier(name)))
                    }
                    _ => None,
                };
                expr = Expr::ArrayRemove {
                    array: Box::new(expr),
                    index,
                    return_val,
                };
                continue;
            }

            let op = match self.current_token {
                Token::Modulo => crate::ast::Operator::Modulo,
                Token::Power => crate::ast::Operator::Power,
                Token::Bang => crate::ast::Operator::Factorial,
                Token::Slash => crate::ast::Operator::Length,
                Token::Underscore => crate::ast::Operator::Floor,
                Token::Caret => crate::ast::Operator::Ceiling,
                Token::Sort => crate::ast::Operator::Sort,
                Token::RevSort => crate::ast::Operator::RevSort,
                Token::Tilde => crate::ast::Operator::Round,
                Token::Backslash => crate::ast::Operator::Avg,
                Token::Less => crate::ast::Operator::Min,
                Token::Greater => crate::ast::Operator::Max,
                _ => unreachable!(),
            };
            self.next_token();
            expr = Expr::UnaryPost {
                op,
                expr: Box::new(expr),
            };
        }

        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match &self.current_token {
            Token::String(s) => {
                let s = s.clone();
                self.next_token();
                Some(Expr::String(s))
            }
            Token::Number(n) => {
                let n = *n;
                self.next_token();
                Some(Expr::Number(n))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.next_token();
                Some(Expr::Identifier(name))
            }
            Token::LParen => {
                self.next_token();
                // Empty tuple ()
                if self.current_token == Token::RParen {
                    self.next_token();
                    return Some(Expr::Tuple(vec![]));
                }
                let first = self.parse_expression()?;
                if self.current_token == Token::Colon {
                    // Single map entry: ("key": val) used for hash++("key": val)
                    self.next_token(); // consume ':'
                    let val = self.parse_expression()?;
                    if self.current_token != Token::RParen {
                        panic!("Expected ')' after map entry");
                    }
                    self.next_token();
                    Some(Expr::Map(vec![(first, val)]))
                } else if self.current_token == Token::Comma {
                    // Tuple: (a, b, ...)
                    let mut elements = vec![first];
                    while self.current_token == Token::Comma {
                        self.next_token();
                        if self.current_token == Token::RParen { break; }
                        if let Some(e) = self.parse_expression() {
                            elements.push(e);
                        }
                    }
                    if self.current_token != Token::RParen {
                        panic!("Expected ')' after tuple elements");
                    }
                    self.next_token();
                    Some(Expr::Tuple(elements))
                } else {
                    // Grouped expression
                    if self.current_token != Token::RParen {
                        panic!("Expected ')' after expression");
                    }
                    self.next_token();
                    Some(first)
                }
            }
            Token::LBracket => {
                self.next_token(); // consume '['
                let mut elements = Vec::new();
                while self.current_token != Token::RBracket && self.current_token != Token::EOF {
                    if let Some(expr) = self.parse_expression() {
                        elements.push(expr);
                    }
                    if self.current_token == Token::Comma {
                        self.next_token(); // consume ','
                    } else {
                        break;
                    }
                }
                if self.current_token != Token::RBracket {
                    panic!("Expected ']' after array elements");
                }
                self.next_token(); // consume ']'
                Some(Expr::Array(elements))
            }
            Token::LBrace => {
                self.next_token(); // consume '{'
                // Empty {} — undetermined, stored as empty set
                if self.current_token == Token::RBrace {
                    self.next_token();
                    return Some(Expr::Set(vec![]));
                }
                let first = self.parse_expression()?;
                if self.current_token == Token::Colon {
                    // Hashmap: { key: val, ... }
                    self.next_token(); // consume ':'
                    let first_val = self.parse_expression()?;
                    let mut pairs = vec![(first, first_val)];
                    while self.current_token == Token::Comma {
                        self.next_token();
                        if self.current_token == Token::RBrace { break; }
                        let key = self.parse_expression()?;
                        if self.current_token != Token::Colon {
                            panic!("Expected ':' in map literal");
                        }
                        self.next_token();
                        let val = self.parse_expression()?;
                        pairs.push((key, val));
                    }
                    if self.current_token != Token::RBrace {
                        panic!("Expected '}}' after map entries");
                    }
                    self.next_token();
                    Some(Expr::Map(pairs))
                } else {
                    // Set: { val, ... }
                    let mut elements = vec![first];
                    while self.current_token == Token::Comma {
                        self.next_token();
                        if self.current_token == Token::RBrace { break; }
                        if let Some(e) = self.parse_expression() {
                            elements.push(e);
                        }
                    }
                    if self.current_token != Token::RBrace {
                        panic!("Expected '}}' after set elements");
                    }
                    self.next_token();
                    Some(Expr::Set(elements))
                }
            }
            // _ is the implicit lambda variable
            Token::Underscore => {
                self.next_token();
                Some(Expr::Identifier("_".to_string()))
            }
            _ => None,
        }
    }
}