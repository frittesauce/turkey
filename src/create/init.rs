use std::{env, io};

use crate::create::setup::setup;

pub fn init() -> io::Result<()> {
    println!("making new project inside folder!");

    let proj_dir = env::current_dir()?;

    setup(&proj_dir);

    return Ok(());
}
