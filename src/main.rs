use std::{
    env,
    fmt::Display,
    fs::OpenOptions,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use lexer::Lexer;

mod lexer;
mod parser;
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
}

/// Instruction manual
static MANUAL: &str = include_str!("manual.txt");

/// Parses arguments
fn parse_args() -> Result<ParsedArgs, AppError> {
    let mut args = env::args();
    args.next();
    if let Some(arg) = args.next() {
        if arg.eq("-i") || arg.eq("--interactive") {
            Ok(ParsedArgs::Run(InputType::Standard))
        } else if arg.eq("-f") || arg.eq("--file") {
            if let Some(path) = args.next() {
                if let Ok(path) = PathBuf::try_from(&path) {
                    Ok(ParsedArgs::Run(InputType::File(path)))
                } else {
                    Err(AppError {
                        msg: format!("Invalid path `{path}`."),
                        code: 1,
                    })
                }
            } else {
                Err(AppError {
                    msg: "Missing input file path argument.".to_owned(),
                    code: 2,
                })
            }
        } else {
            Err(AppError {
                msg: format!("Invalid argument `{}`.", arg),
                code: 3,
            })
        }
    } else {
        Ok(ParsedArgs::InstructionManual)
    }
}

/// Consumes and prints all lexems
fn print_lexems(lexer: &mut Lexer) {
    for token in lexer.all() {
        println!("{}", token);
    }
}

/// Prints all errors
fn print_errors(lexer: &Lexer) {
    for e in &lexer.errors {
        eprintln!("{}", e);
    }
}

/// Application error containing message and process return code
struct AppError {
    msg: String,
    code: u8,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.msg))
    }
}

/// Entry point
fn main() {
    if let Err(e) = app() {
        eprintln!("{}", e.msg);
        std::process::exit(e.code as i32);
    } else {
        std::process::exit(0);
    }
}

/// Run application
fn app() -> Result<(), AppError> {
    match parse_args() {
        Ok(ParsedArgs::InstructionManual) => {
            println!("{MANUAL}");
            Ok(())
        }
        Ok(ParsedArgs::Run(input)) => run(input),
        Err(e) => Err(e),
    }
}

/// Run interpreter
fn run(input: InputType) -> Result<(), AppError> {
    let reader: Box<dyn BufRead> = match input {
        InputType::Standard => Box::new(BufReader::new(stdin())),
        InputType::File(path) => {
            if let Ok(input_file) = OpenOptions::new().read(true).open(&path) {
                Box::new(BufReader::new(input_file))
            } else {
                return Err(AppError {
                    msg: format!("No file found `{}`.", path.to_string_lossy()),
                    code: 1,
                });
            }
        }
    };
    let mut lexer = Lexer::new(reader);

    print_lexems(&mut lexer); // TEMPORARY

    print_errors(&lexer);

    Ok(())
}
