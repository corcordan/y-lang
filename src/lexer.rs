#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Identifier(String),

    Plus,
    Minus,
    Star,
    Slash,
    DoubleSlash,
    Modulo,
    Power,
    Bang,
    PipeArrow,
    Increment,
    Decrement,

    Assign,
    Equal,
    NotEqual,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    And,
    Or,
    Xor,
    Nand,
    Nor,
    Xnor,

    Ampersand,
    Pipe,

    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LAngle,
    RAngle,

    Comma,
    Colon,
    Backslash,
    Hash,
    Underscore,
    Caret,
    At,

    EOF,
}

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    // Create a new lexer from the source code
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            position: 0,
        }
    }

    // Read the next character and advance the position
    fn read_next_char(&mut self) -> Option<char> {
        if self.position >= self.source.len() {
            None
        } else {
            let ch = self.source[self.position];
            self.position += 1;
            Some(ch)
        }
    }

    // Get the next token from the input
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(ch) = self.read_next_char() {
            match ch {
                '|' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '>' {
                            self.read_next_char();
                            Token::PipeArrow
                        } else if next_ch == '|' {
                            self.read_next_char();
                            Token::Or
                        }
                        else {
                            Token::Pipe
                        }
                    } else {
                        Token::Pipe
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    // Put the first letter back and read the full identifier
                    self.position -= 1;
                    self.read_identifier()
                }
                '"' => self.read_string(),
                '\'' => self.read_char(),
                '0'..='9' => {
                    // Put the first digit back and read the full number
                    self.position -= 1;
                    self.read_number()
                }
                '#' => self.skip_comment(),
                '+' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '+' {
                            self.consume_char();
                            Token::Increment
                        } else {
                            Token::Plus
                        }
                    } else {
                        Token::Plus
                    }
                }
                '-' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '-' {
                            self.consume_char();
                            Token::Decrement
                        } else {
                            Token::Minus
                        }
                    } else {
                        Token::Minus
                    }
                }
                '*' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '*' {
                            self.consume_char();
                            Token::Power
                        } else {
                            Token::Star
                        }
                    } else {
                        Token::Star
                    }
                }
                '/' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '/' {
                            self.consume_char();
                            Token::DoubleSlash
                        } else {
                            Token::Slash
                        }
                    } else {
                        Token::Slash
                    }
                }
                '%' => Token::Modulo,
                '>' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '=' {
                            self.consume_char();
                            Token::GreaterEqual
                        } else {
                            Token::Greater
                        }
                    } else {
                        Token::Greater
                    }
                }
                '<' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '=' {
                            self.consume_char();
                            Token::LessEqual
                        } else {
                            Token::Less
                        }
                    } else {
                        Token::Less
                    }
                }
                '(' => Token::LParen,
                ')' => Token::RParen,
                '!' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '=' {
                            self.consume_char();
                            Token::NotEqual
                        } 
                        else if next_ch == '&' {
                            self.consume_char();
                            Token::Nand
                        }
                        else if next_ch == '|' {
                            self.consume_char();
                            Token::Nor
                        }
                        else if next_ch == '^' {
                            self.consume_char();
                            Token::Xnor
                        }
                        else {
                            Token::Bang
                        }
                    } else {
                        Token::Bang
                    }
                }
                '=' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '=' {
                            self.consume_char();
                            Token::Equal
                        } else {
                            Token::Assign
                        }
                    } else {
                        Token::Assign
                    }
                }
                ',' => Token::Comma,
                ':' => Token::Colon,
                '_' => Token::Underscore,
                '^' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '^' {
                            self.consume_char();
                            Token::Xor
                        } else {
                            Token::Caret
                        }
                    } else {
                        Token::Caret
                    }
                }
                '&' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '&' {
                            self.consume_char();
                            Token::And
                        } else {
                            Token::Ampersand
                        }
                    } else {
                        Token::Ampersand
                    }
                }
                '\\' => Token::Backslash, 
                _ => panic!("Unexpected character: {}", ch),
            }
        } else {
            Token::EOF
        }
    }

    // Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.consume_char();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) -> Token {
        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break;
            } else {
                self.consume_char();
            }
        }
        self.next_token()
    }

    // Read an identifier (not used in this simple example, but can be extended for variables, etc.)
    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() {
                ident.push(ch);
                self.consume_char();
            } else {
                break;
            }
        }
        Token::Identifier(ident)
    }

    // Read a number (integers and floating point)
    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        let mut has_decimal = false;
        
        while let Some(ch) = self.peek_char() {
            if ch.is_digit(10) {
                number.push(ch);
                self.consume_char();
            } else if ch == '.' && !has_decimal {
                // Allow one decimal point
                has_decimal = true;
                number.push(ch);
                self.consume_char();
            } else {
                break;
            }
        }
        Token::Number(number.parse().expect("Failed to parse number"))
    }

    // Read a string literal
    fn read_string(&mut self) -> Token {
        let mut string = String::new();
        while let Some(ch) = self.read_next_char() {
            if ch == '"' {
                break;
            } else {
                string.push(ch);
            }
        }
        Token::String(string)
    }

    // Read a char literal with '
    fn read_char(&mut self) -> Token {
        // Read the character after the opening '
        if let Some(ch) = self.read_next_char() {
            // Check if it's immediately a closing quote (empty char)
            if ch == '\'' {
                Token::String(String::new())
            } else {
                // Check for the closing '
                if let Some(closing_quote) = self.read_next_char() {
                    if closing_quote == '\'' {
                        Token::String(ch.to_string())
                    } else {
                        panic!("Expected closing ' after character literal");
                    }
                } else {
                    panic!("Unexpected end of input in character literal");
                }
            }
        } else {
            panic!("Unexpected end of input after opening '");
        }
    }

    // Advance position by one character (consume without returning)
    fn consume_char(&mut self) {
        self.position += 1;
    }

    // Peek at the next character without advancing the position
    fn peek_char(&self) -> Option<char> {
        if self.position >= self.source.len() {
            None
        } else {
            Some(self.source[self.position])
        }
    }
}