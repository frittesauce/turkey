use std::{fs, process};

mod lexer;

pub fn compile(path: &str) {
    let content = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Error reading {}: {}", path, err);
        process::exit(1);
    });

    println!("read the file: {}", content)
}
