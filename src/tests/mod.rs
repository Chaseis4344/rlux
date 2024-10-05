use crate::run;
use std::{fs, path::Path, process::exit};

#[cfg(test)]
#[test]
fn control_test() {
    run_file_test(String::from("./src/tests/control_test.lux"));
}

#[test]
fn scope_test() {
    run_file_test(String::from("./src/tests/scope_test.lux"));
}

#[test]
fn syntax_test() {
    run_file_test(String::from("./src/tests/syntax_test.lux"));
}

fn run_file_test(filepath: String) {
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
            println!("Please provide a lux source file");
        }
    }

    let source = fs::read_to_string(file_path);

    //Check source for OS Errors
    if source.is_err() {
        let error = source.unwrap_err();
        eprintln!("File Error: {}", error);
        exit(1);
    }

    let valid_source = source.unwrap();

    //Run the code
    match run(valid_source) {
        Ok(_) => {
            //exit(0);
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
}
