mod scanner;

use std::any::Any;
use std::env::{args, args_os};
use std::fs;
use std::io::stdin;
use std::process::exit;

fn main() {
    // TODO: convert to match
    if args().len() > 2 {
        eprintln!("Usage: rslox [script]");
        exit(64);
    } else if args().len() == 2 {
        run_file(args().nth(1).unwrap())
    } else {
        run_prompt()
    }
}

fn run_file(path: String) {
    match fs::read_to_string(path) {
        Ok(source) => run(source),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            exit(74);
        }
    }
}

fn run_prompt() {
    let mut input = String::new();
    loop {
        println!("> ");
        match stdin().read_line(&mut input) {
            Ok(_) => run(input.clone()),
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                exit(74);
            }
        }
    }
}

fn run(source: String) {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

