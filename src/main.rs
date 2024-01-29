use std::{
    cmp::Ordering,
    env, fs,
    io::{self, BufRead},
};

use scanner::Scanner;
use token::Token;

use crate::lox_error::LoxError;

mod lox_error;
mod scanner;
mod token;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    match args.len().cmp(&2) {
        Ordering::Greater => {
            print!("Usage: lox-rs [script]");
            panic!()
        }
        Ordering::Equal => {
            run_file(args.pop().unwrap());
        }
        _ => run_prompt(),
    }
}

fn run_file(path: String) {
    let bytes: Vec<u8> = fs::read(path).unwrap();
    let string_bytes = String::from_utf8(bytes).unwrap();
    run(string_bytes);
}

fn run_prompt() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        print!("> ");
        run(line.unwrap());
    }
}

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    tokens.iter().for_each(|token| println!("{}", token));
}
