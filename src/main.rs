mod args;
mod build;
mod create;

use args::CmdArgs;
use clap::Parser;

use crate::build::build;
use crate::create::init::init;
use crate::create::new::new;

fn main() {
    let args = CmdArgs::parse();

    match args.command {
        args::Cmd::New(commands) => {
            let res = new(&commands.name);
            match res {
                Err(error) => {
                    eprintln!("{:#}", error);
                }
                Ok(()) => {
                    println!("made new project succesfully!")
                }
            }
        }

        args::Cmd::Build { path } => {
            let path = match path {
                Some(p) => p,
                None => std::env::current_dir().unwrap(),
            };
            let res = build(&path);
            println!("{:?}", res.unwrap())
        }

        args::Cmd::Init => {
            let res = init();
            match res {
                Err(error) => {
                    eprintln!("{:#}", error);
                }
                Ok(()) => {
                    println!("made new project succesfully!")
                }
            }
        }
    }
}
