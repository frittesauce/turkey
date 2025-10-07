use std::{io, path::Path};

pub fn build(path: &Path) -> io::Result<&str> {
    println!("compiling started at {:?}!", path);

    let config_file_path = path.join("turkey.toml");
    let main_file_path = path.join("src/main.tky");
    if !config_file_path.exists() {
        return Ok("config file doesnt exist !");
    }

    if !main_file_path.exists() {
        return Ok("main.tky doesnt exist!");
    }

    return Ok("compiled");
}
