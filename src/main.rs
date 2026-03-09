use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let source = fs::read_to_string(filename).expect("Could not read file");

    println!("Source code:\n{}", source);
}