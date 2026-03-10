use std::env;
use std::fs;
mod lexer;
use lexer::{Lexer, Token};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let source = fs::read_to_string(filename).expect("Could not read file");

    println!("Source code:\n{}", source);

    let mut lexer = Lexer::new(source);
    loop {
        let token = lexer.next_token();
        match token {
            Token::EOF => break,
            _ => println!("Token: {:?}", token),
        }
    }
}