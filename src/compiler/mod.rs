use std::{fs, io, path::Path};

use crate::compiler::lexer::lexer;

mod lexer;

pub fn compile(path: &Path) -> io::Result<&str> {
    println!("compiling started at {:?}!", path);

    let config_file_path = path.join("turkey.toml");
    let main_file_path = path.join("src/main.tky");
    if !config_file_path.exists() {
        return Ok("config file doesnt exist !");
    }

    if !main_file_path.exists() {
        return Ok("main.tky doesnt exist!");
    }

    let main_content = fs::read_to_string(main_file_path)?;

    lexer(&main_content);

    return Ok("string");
}
