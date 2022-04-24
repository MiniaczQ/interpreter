//#![feature(trace_macros)]
//trace_macros!(true);

use std::{
    env,
    fs::OpenOptions,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use lexer::Lexer;

mod lexer;
mod position;
mod scannable;

/// Source of code
enum InputType {
    Standard,
    File(PathBuf),
}

/// Information about execution derived from input parameters
enum ParsedArgs {
    InstructionManual,
    Run(InputType),
    Error(String),
}

/// Instruction manual
static MANUAL: &str = include_str!("manual.txt");

/// Parses arguments
fn parse_args() -> ParsedArgs {
    let mut args = env::args();
    args.next();
    if let Some(arg) = args.next() {
        if arg.eq("-i") || arg.eq("--interactive") {
            ParsedArgs::Run(InputType::Standard)
        } else if arg.eq("-f") || arg.eq("--file") {
            if let Some(path) = args.next() {
                if let Ok(path) = PathBuf::try_from(&path) {
                    ParsedArgs::Run(InputType::File(path))
                } else {
                    ParsedArgs::Error(format!("Invalid path `{path}`"))
                }
            } else {
                ParsedArgs::Error("Missing input file path argument.".to_owned())
            }
        } else {
            ParsedArgs::Error(format!("Invalid argument `{}`", arg))
        }
    } else {
        ParsedArgs::InstructionManual
    }
}

/// Consumes and prints all lexems
fn print_lexems(lexer: &mut Lexer) {
    for token in lexer {
        println!("{}", token);
    }
}

/// Entry point
fn main() {
    match parse_args() {
        ParsedArgs::InstructionManual => println!("{MANUAL}"),
        ParsedArgs::Error(msg) => println!("{msg}"),
        ParsedArgs::Run(input) => {
            let reader: Box<dyn BufRead> = match input {
                InputType::Standard => Box::new(BufReader::new(stdin())),
                InputType::File(path) => Box::new(BufReader::new(
                    OpenOptions::new()
                        .read(true)
                        .open(path)
                        .expect("Input file not found."),
                )),
            };
            let mut lexer = Lexer::new(reader);
            print_lexems(&mut lexer);
        }
    }
}
