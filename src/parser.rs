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
        match &self.current_token {
            Token::Print => {
                self.next_token();
                Some(Stmt::Print(self.parse_expression()?))
            }
            Token::String(_) => {
                let expr = self.parse_expression()?;
                // Handle pipe arrow if present
                if self.current_token == Token::PipeArrow {
                    self.next_token();
                    // After pipe arrow, if we have Print, parse it
                    if self.current_token == Token::Print {
                        self.next_token();
                        return Some(Stmt::Print(expr));
                    }
                }
                Some(Stmt::Expression(expr))
            }
            Token::EOF => None,
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    // Parse an expression (currently only string literals)
    fn parse_expression(&mut self) -> Option<Expr> {
        match &self.current_token {
            Token::String(s) => {
                let s = s.clone();
                self.next_token();
                Some(Expr::String(s))
            }
            _ => None,
        }
    }

    // Placeholder methods for future parsing features
    fn parse_pipe_expression(&mut self) {
        // This is a placeholder for future pipe expression parsing
        panic!("Pipe expression parsing not implemented yet");
    }

    // Placeholder methods for future parsing features
    fn parse_assign_statement(&mut self) {
        // This is a placeholder for future variable assignment parsing
        panic!("Variable assignment not implemented yet");
    }

    // Placeholder methods for future parsing features
    fn parse_identifier(&mut self) {
        // This is a placeholder for future identifier parsing
        panic!("Identifier parsing not implemented yet");
    }

    // Placeholder methods for future parsing features
    fn parse_number(&mut self) {
        // This is a placeholder for future number parsing
        panic!("Number parsing not implemented yet");
    }

    // Placeholder methods for future parsing features
    fn parse_string(&mut self) {
        // This is a placeholder for future string parsing
        panic!("String parsing not implemented yet");
    }

    // Placeholder methods for future parsing features
    fn parse_infix_expression(&mut self) {
        // This is a placeholder for future infix expression parsing
        panic!("Infix expression parsing not implemented yet");
    }
}