use std::{env, path::Path, process};

use crate::compiler::compile;

pub mod compiler;

fn main() {
    println!("welcome to turkey! dont use this language!");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    compile(filename);
}
