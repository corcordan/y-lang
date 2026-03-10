use std::env;
use std::fs;
mod lexer;
mod parser;
mod ast;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let source = fs::read_to_string(filename).expect("Could not read file");

    println!("Source code:\n{}", source);

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    println!("\nParsed program:");
    for stmt in program {
        println!("{:?}", stmt);
    }
}