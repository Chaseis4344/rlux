use std::{
    env, fs,
    io::{stdin, Error},
    process::exit,
};

//mod interpreter;
mod enviroment;
mod parser;
mod scanner;
mod tests;
mod types;
//mod token;
//mod display_traits;
//sends error report to user with specific additonal details
fn report(line: u32, place: String, message: String) -> Result<i32, Error> {
    eprintln!(" [Line {}]Error{}: {}", line, place, message);
    let return_err = Error::new(
        std::io::ErrorKind::InvalidData,
        format!(" [Line {}]Error{}: {}", line, place, message),
    );
    Result::Err(return_err)
}

//Sends an error report to user - semantic sugar
fn error(line: u32, message: String) -> Result<i32, Error> {
    report(line, String::from(""), message)
    //exit(65);
}

//Runs source string provided, may be multi-line string
fn run(source: String) -> Result<i32, Error> {
    let mut scanner = scanner::Scanner::new(source, None, None);
    //Scan in & Store token string
    let mut tokens: Vec<types::token::Token> = scanner.scan_tokens();

    //Push Final EOF token
    tokens.push(types::token::Token {
        token_type: types::TokenType::Eof,
        lexeme: String::from(""),
        literal: None,
        line: scanner.line,
    });

    /*Parser goes here */
    let mut parser = parser::Parser::new(tokens, 0);
    let statements = parser.parse();
    let mut interpreter = parser::interpreter::Interpreter::new();
    interpreter.interpret(statements);

    Result::Ok(65)
}

use std::ffi::OsStr;
use std::path::Path;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

//On Startup - Runs source from provided filepath
pub fn run_file(filepath: String) {
    let source = fs::read_to_string(filepath);
    //Check source for OS Errors
    if source.is_err() {
        let error = source.unwrap_err();
        eprintln!("File Error: {}", error);
        exit(65);
    }

    let valid_source = source.unwrap();

    //Check that we get an extension & its the right one
    let invalid_extension = get_extension_from_filename(&valid_source);
    if invalid_extension.is_none() {
        println!("Please provide a lox source file.");
        exit(65);
    }

    let valid_extension = invalid_extension.unwrap();
    if valid_extension != "lox" {
        println!("Please provide a lox source file.");
        exit(65);
    }

    //Run the code
    match run(valid_source) {
        Ok(number) => {
            exit(number);
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(65);
        }
    };
}

//On startup - Enters Interactive Mode
pub fn run_prompt() {
    loop {
        let input: &mut String = &mut String::new();
        let matcher = stdin().read_line(input);

        //Bad Path 1
        if matcher.is_err() {
            let err = matcher.unwrap_err();
            eprintln!("{}", err);
            exit(65);
        }

        //Core function of REPL
        let result = run(input.to_string());

        // Bad Path 2
        if result.is_err() {
            let err = result.unwrap_err();
            eprintln!("{}", err);
            exit(65);
        }

        //Check if we exit Normally
        let number = result.unwrap();
        if number == 0 {
            exit(64);
        }
    }
}

fn main() {
    //Collect arguments then run based on number of arguments
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => run_file(args[1].clone()),
        1 => run_prompt(),
        _ => {
            println!("Usage: rlux [script]");
            exit(64);
        }
    }
}
