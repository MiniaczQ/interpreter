use std::{
    env,
    fmt::Display,
    fs::OpenOptions,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

use lexer::{lexem::LexerWarning, Lexer};
use parser::{
    grammar::program::Program, token_scanner::TokenScanner, Parser, ParserError, ParserWarning,
};

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

/// Run a lexer and parser on some arbitrary reader
fn parse(
    reader: Box<dyn BufRead>,
) -> (
    Result<Program, ParserError>,
    Vec<ParserWarning>,
    Vec<LexerWarning>,
) {
    let mut lexer = Lexer::new_with_defaults(reader);

    let (result, parser_warnings) = {
        let mut parser = Parser::new_with_defaults(TokenScanner::new(&mut lexer));
        let result = parser.parse();
        let warnings = parser.get_warnings();
        (result, warnings)
    };

    let lexer_warnings = lexer.get_warnings();
    (result, parser_warnings, lexer_warnings)
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

    let (result, parser_warnings, lexer_warnings) = parse(reader);

    match result {
        Ok(program) => println!("{}", program),
        Err(error) => eprintln!("{}", error),
    }

    for w in parser_warnings {
        eprintln!("{}", w);
    }

    for w in lexer_warnings {
        eprintln!("{}", w);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs::OpenOptions, io::BufReader};

    use crate::{
        lexer::lexem::{LexerWarning, LexerWarningVariant},
        parse,
        parser::{
            grammar::program::Program, ParserError, ParserErrorVariant, ParserWarning,
            ParserWarningVariant,
        },
    };

    fn read(
        path: &str,
    ) -> (
        Result<Program, ParserError>,
        Vec<ParserWarning>,
        Vec<LexerWarning>,
    ) {
        let file = OpenOptions::new().read(true).open(&path).unwrap();
        let reader = Box::new(BufReader::new(file));
        parse(reader)
    }

    #[test]
    fn string() {
        let string = "// do nothing\nfn main() {\n    let a: int = 5;\n}";
        let reader = Box::new(BufReader::new(string.as_bytes()));
        let (res, par_warns, lex_warns) = parse(reader);
        assert!(res.is_ok());
        assert!(par_warns.is_empty());
        assert!(lex_warns.is_empty());
    }

    #[test]
    fn short() {
        let (res, par_warns, lex_warns) = read("snippets/short.txt");
        assert!(res.is_ok());
        assert!(par_warns.is_empty());
        assert!(lex_warns.is_empty());
    }

    #[test]
    fn long() {
        let (res, par_warns, lex_warns) = read("snippets/long.txt");
        assert!(res.is_ok());
        assert!(par_warns.is_empty());
        assert!(lex_warns.is_empty());
    }

    #[test]
    fn errors() {
        let (res, par_warns, lex_warns) = read("snippets/parser_error.txt");
        assert_eq!(
            res.unwrap_err().error,
            ParserErrorVariant::VariableDeclarationMissingType
        );
        assert_eq!(par_warns.len(), 1);
        assert_eq!(
            par_warns[0].warning,
            ParserWarningVariant::VariableDeclarationMissingTypeSeparator
        );
        assert_eq!(lex_warns.len(), 1);
        assert_eq!(
            lex_warns[0].warning,
            LexerWarningVariant::InvalidSequence("#$@".to_owned())
        );
    }

    #[test]
    fn warnings() {
        let (res, par_warns, lex_warns) = read("snippets/warnings.txt");
        assert!(res.is_ok());
        assert_eq!(par_warns.len(), 1);
        assert_eq!(
            par_warns[0].warning,
            ParserWarningVariant::VariableDeclarationMissingTypeSeparator
        );
        assert_eq!(lex_warns.len(), 1);
        assert_eq!(
            lex_warns[0].warning,
            LexerWarningVariant::InvalidSequence("#$@".to_owned())
        );
    }
}
