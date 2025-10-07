use std::{fs, io, path::Path};

use crate::create::setup::setup;

pub fn new(name: &str) -> io::Result<()> {
    println!("making new project named {}", name);

    let proj_dir = Path::new(name);

    if proj_dir.exists() {
        eprintln!(
            "project directory already exists in current directory! run turkey init inside it or try a diffrent name!"
        );
        return Ok(());
    }
    let _ = fs::create_dir(proj_dir);

    setup(proj_dir);

    return Ok(());
}
