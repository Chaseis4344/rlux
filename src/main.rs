use std::{
    env, fs,
    io::{stdin, Error},
    process::exit,
};

pub mod ast;
mod display_traits;
mod parser;
mod scanner;
//mod tests;
mod token;
mod types;

//sends error report to user with specific additonal details
fn report(line: u32, place: String, message: String) -> Result<i32, Error> {
    eprintln!(" [Line {}]Error{}: {}", line, place, message);
    Result::Err(Error::new(
        std::io::ErrorKind::InvalidData,
        format!(" [Line {}]Error{}: {}", line, place, message),
    ))
}

//Sends an error report to user - semantic sugar
fn error(line: u32, message: String) -> Result<i32, Error> {
    report(line, String::from(""), message)
    //exit(65);
}

fn run(source: String) -> Result<i32, Error> {
    let mut scanner = scanner::Scanner::new(source, None, None, None);

    let mut tokens: Vec<token::Token> = scanner.scan_tokens();

    tokens.push(token::Token {
        token_type: types::TokenType::Eof,
        lexeme: String::from(""),
        literal: None,
        line: scanner.line,
    });

    for token in tokens {
        print!(">");
        println!("{}", token);
    }

    Result::Ok(65)
}

use std::ffi::OsStr;
use std::path::Path;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn run_file(filepath: String) {
    let source = fs::read_to_string(filepath);
    match source {
        Ok(valid_source) => match get_extension_from_filename(&valid_source) {
            Some(extension) => {
                //Make sure we have the right extension
                if extension != "lox" {
                    println!("Please provide a lox source file.");
                    exit(65);
                }

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
            None => {
                println!("Please provide a lox source file.");
                exit(65);
            }
        },
        Err(error) => {
            eprintln!("File Error: {}", error);
            exit(65);
        }
    }
}

pub fn run_prompt() {
    loop {
        let input: &mut String = &mut String::new();
        //Fancyschmancy stuff
        //print!("> ");
        // stdin().read_line(input);
        let matcher = stdin().read_line(input);
        //Read input & break on error
        match matcher {
            Ok(_) => {
                match run(input.to_string()) {
                    Ok(number) => {
                        //exit(number);
                        if number == 0 {
                            exit(64)
                        } else {
                            continue;
                        }
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(65);
                    }
                };
            }
            Err(_) => {}
        }

        //Evaluate & Print

        //Reset Buffer, all code is run line-by-line, no memory
    }
}

fn main() {
    //Collect arguments then run based on number of arguments
    let args: Vec<String> = env::args().collect();
    //dbg!(args.clone());

    match args.len() {
        2 => run_file(args[1].clone()),
        1 => run_prompt(),
        _ => {
            println!("Usage: rlux [script]");
            exit(64);
        }
    }
}
