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
        let mut expr = self.parse_primary()?;
        
        // Handle pipe operations: expr |> function
        while self.current_token == Token::Pipe {
            self.next_token(); // consume '|>'
            let function = self.parse_primary()?;
            expr = Expr::Call {
                callee: Box::new(function),
                args: vec![expr],
            };
        }
        
        Some(expr)
    }

    fn parse_equality(&mut self) {
        // This is a placeholder for future equality expression parsing
        panic!("Equality expression parsing not implemented yet");
    }

    fn parse_comparison(&mut self) {
        // This is a placeholder for future comparison expression parsing
        panic!("Comparison expression parsing not implemented yet");
    }

    fn parse_term(&mut self) {
        // This is a placeholder for future comparison expression parsing
        panic!("Comparison expression parsing not implemented yet");
    }

    fn parse_factor(&mut self) {
        // This is a placeholder for future comparison expression parsing
        panic!("Comparison expression parsing not implemented yet");
    }

    fn parse_unary(&mut self) {
        // This is a placeholder for future unary expression parsing
        panic!("Unary expression parsing not implemented yet");
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

    //----------------------------------------------------------------------

    fn parse_lambda(&mut self) {
        // This is a placeholder for future lambda expression parsing
        panic!("Lambda expression parsing not implemented yet");
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