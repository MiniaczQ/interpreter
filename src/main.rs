use std::{fs::OpenOptions, io::BufReader};

use lexer::Lexer;

mod lexer;
mod macros;
mod matchers;
mod position;
mod scanner;
mod token;

fn main() {
    let file = OpenOptions::new().read(true).open("test.txt").unwrap();
    let parser = Lexer::new(BufReader::new(file));
    for token in parser {
        println!("{:?}", token);
    }
}
