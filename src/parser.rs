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
        while self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }
        statements
    }

    // Parse a single statement
    fn parse_statement(&mut self) -> Option<Stmt> {
        self.parse_expression().map(Stmt::Expression)
    }

    // Parse an expression (handles pipe operations)
    fn parse_expression(&mut self) -> Option<Expr> {
        let mut expr = self.parse_assignment()?;
        
        // Handle pipe operations: expr |> function
        while self.current_token == Token::PipeArrow {
            self.next_token(); // consume '|>'
            let function = self.parse_primary()?;
            expr = Expr::Call {
                callee: Box::new(function),
                args: vec![expr],
            };
        }
        
        Some(expr)
    }

    fn parse_assignment(&mut self) -> Option<Expr> {
        let expr = self.parse_or()?;

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
            Token::Increment => {
                self.next_token();
                let expr = self.parse_unary()?;
                Some(Expr::UnaryPre {
                    op: crate::ast::Operator::Increment,
                    expr: Box::new(expr),
                })
            }
            Token::Decrement => {
                self.next_token();
                let expr = self.parse_unary()?;
                Some(Expr::UnaryPre {
                    op: crate::ast::Operator::Decrement,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        while matches!(self.current_token, Token::Increment | Token::Decrement | Token::Modulo | Token::Power | Token::Bang | Token::Slash | Token::Underscore | Token::Caret) {
            // when encountering slash, power, or modulo, ensure we aren't
            // looking at a binary operator (i.e. another expression follows)
            if matches!(self.current_token, Token::Slash | Token::Power | Token::Modulo) {
                match self.peek_token {
                    Token::Number(_)
                    | Token::String(_)
                    | Token::Identifier(_)
                    | Token::LParen
                    | Token::Minus
                    | Token::Bang
                    | Token::Plus
                    | Token::Increment
                    | Token::Decrement => break,
                    _ => {}
                }
            }

            let op = match self.current_token {
                Token::Increment => crate::ast::Operator::Increment,
                Token::Decrement => crate::ast::Operator::Decrement,
                Token::Modulo => crate::ast::Operator::Modulo,
                Token::Power => crate::ast::Operator::Power,
                Token::Bang => crate::ast::Operator::Factorial,
                Token::Slash => crate::ast::Operator::Length,
                Token::Underscore => crate::ast::Operator::Floor,
                Token::Caret => crate::ast::Operator::Ceiling,
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
                let expr = self.parse_expression();
                if self.current_token != Token::RParen {
                    panic!("Expected ')' after expression");
                }
                self.next_token();
                expr
            }
            _ => None,
        }
    }
}