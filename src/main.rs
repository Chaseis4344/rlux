use std::{
    env, fs,
    io::{Error, stdin},
    process::exit,
};

//Execution Path Modules
mod interpreter;
mod ir;
mod parser;
mod scanner;

//Misc Utility Modules
mod enviroment;
mod types;

//Meta Modules
mod macros;
#[cfg(test)]
mod tests;

use macros::debug;

enum ExitCode {
    Okay = 0,
    GenerallyBad = 1,
    CommandLineErr = 64,
    DataErr = 65,
    OSErr = 72,
}

#[allow(unused)]
///Sends runtime error report to user with specific additonal details
fn report(line: u32, place: String, message: String) -> Error {
    if place != *"" {
        let place: String = "in ".to_owned() + &place;
    }

    eprintln!(" [Line {line}]Error: {message} {place}");

    Error::new(
        std::io::ErrorKind::InvalidData,
        format!(" [Line {line}]Error: {message} {place}"),
    )
}

///Sends an error report to user - semantic sugar
fn error(line: u32, message: String) -> Error {
    report(line, String::from(""), message)
}

///Runs source string provided, may be multi-line string
fn run(source: String) -> Result<i32, Error> {
    let mut scanner = scanner::Scanner::new(source, None, Some(1));

    //Scan in & Store token string
    let mut tokens: Vec<types::token::Token> = scanner.scan_tokens();

    //Push Final EOF token
    tokens.push(types::token::Token {
        token_type: types::TokenType::Eof,
        lexeme: String::from(""),
        literal: None,
        line: scanner.line,
    });

    debug!("Tokenizing Done");

    let mut parser = parser::Parser::new(tokens, 0);
    let statements: Vec<types::statement::Statement> = parser.parse();

    debug!("Parsing Done");

    let ir = ir::statements_to_ir(statements);

    dbg!("{}", ir);

    // let mut interpreter = interpreter::Interpreter::new();
    // interpreter.interpret_ir(ir);

    Result::Ok(ExitCode::Okay as i32)
}

use std::path::Path;

///On Startup - Runs source from provided filepath
pub fn run_file(filepath: String) {
    let file_path = Path::new(&filepath);
    //println!("File Path: {}", filepath);
    if !file_path.exists() {
        println!("Please provide a valid file.");
        return;
    }

    let unvalidated_extension = file_path.extension();
    if unvalidated_extension.is_some() {
        let extension = unvalidated_extension.unwrap();

        if extension != "lux" {
            //This panic is for user safety, we don't want to parse or compile the wrong type of
            //file
            panic!("Please provide a lux source file");
        }
    }

    let source = fs::read_to_string(file_path);

    //Check source for OS Errors
    if source.is_err() {
        let error = source.unwrap_err();
        println!("File Error: {error}");
        exit(ExitCode::GenerallyBad as i32);
    }

    let valid_source = source.unwrap();

    //Run the code
    match run(valid_source) {
        Ok(_) => {
            // exit(ExitCode::Okay as i32);
        }
        Err(err) => {
            println!("{err}");
            // exit(ExitCode::GenerallyBad as i32);
        }
    };
}

///On startup - Enters Interactive Mode
pub fn run_prompt() {
    loop {
        let input: &mut String = &mut String::new();
        let matcher = stdin().read_line(input);

        //Bad Path 1
        if matcher.is_err() {
            let err = matcher.unwrap_err();
            println!("{err}");
            exit(ExitCode::DataErr as i32);
        }

        //Core function of REPL
        let result = run((*input).to_string());

        // Bad Path 2
        if result.is_err() {
            let err = result.unwrap_err();
            println!("{err}");
            exit(ExitCode::DataErr as i32);
        }

        //Check if we exit Normally
        let number = result.unwrap();
        if number == 0 {
            // exit(ExitCode::CommandLineErr as i32);
        }
    }
}

use clap::Parser;
///A small language focused on learning and fun
#[derive(Parser, Debug)]
#[command(version, about, author)]
struct Args {
    ///If this flag is set, instead of being interpreted input will be compiled into llvm
    #[arg(short, long, default_value_t = true)]
    ir: bool,

    ///If this flag set, rlux will enter an interactive REPL
    #[arg(short = 'n', long, default_value_t = false)]
    interpret: bool,

    ///Filepath for .lux source file
    #[arg(short, long)]
    filepath: Option<String>,
}

fn main() {
    //Collect arguments then run based on number of arguments
    // let args: Vec<String> = env::args().collect();
    let args = Args::parse();

    //Runs file then an interactive prompt depending on command-line flags
    //TODO: Intoduce a feature where the file can pass it's enviroment ot the interacttive shell,
    //like Python

    if let Some(filepath) = args.filepath {
        run_file(filepath);
    }

    if args.interpret {
        run_prompt();
    }

    exit(0);
}
