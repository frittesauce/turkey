use std::{fs, process};

use crate::compiler::{lexer::lexer, parser::pars};

mod lexer;
mod parser;

pub fn compile(path: &str) {
    let content = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Error reading {}: {}", path, err);
        process::exit(1);
    });

    let tokens = lexer(&content);

    let ast = parser(tokens);
}
