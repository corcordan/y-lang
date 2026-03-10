#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    String(String),
    PipeArrow,
    Print,
    EOF
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
    fn read_char(&mut self) -> Option<char> {
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

        if let Some(ch) = self.read_char() {
            match ch {
                '|' => {
                    if let Some(next_ch) = self.peek_char() {
                        if next_ch == '>' {
                            self.read_char();
                            Token::PipeArrow
                        } else {
                            panic!("Unexpected character after |: {}", next_ch);
                        }
                    } else {
                        panic!("Unexpected end of input after |");
                    }
                }
                'p' => Token::Print,
                '"' => self.read_string(),
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
                self.read_char();
            } else {
                break;
            }
        }
    }

    // Read an identifier (not used in this simple example, but can be extended for variables, etc.)
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() {
                ident.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        ident
    }

    // Read a number (not used in this simple example, but can be extended for numeric literals)
    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_digit(10) {
                number.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        number
    }

    // Read a string literal
    fn read_string(&mut self) -> Token {
        let mut string = String::new();
        while let Some(ch) = self.read_char() {
            if ch == '"' {
                break;
            } else {
                string.push(ch);
            }
        }
        Token::String(string)
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