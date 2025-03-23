#[cfg(test)]
use crate::run;
use std::{fs, path::Path};

const PREFIX: &'static str = "./src/tests/";

#[allow(dead_code)]
fn run_file_test(filepath: &str) {
    let files = &(format!("{PREFIX}{filepath}"));

    let file_path = Path::new(files);
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
        panic!("File Error: {error}");
    }

    let valid_source = source.expect("Expected Valid Source");

    //Run the code
    match run(valid_source) {
        Ok(_) => {
            //exit(0);
        }
        Err(err) => {
            panic!("{err}");
        }
    };
}

#[test]
fn control_test() {
    run_file_test("control_test.lux");
}

#[test]
fn scope_test() {
    run_file_test("scope_test.lux");
}

#[test]
fn math_test() {
    run_file_test("math_test.lux");
}

#[test]
fn for_loop_test() {
    run_file_test("for_loop_test.lux")
}

#[test]
fn comparison_test() {
    run_file_test("comparison_test.lux")
}
