mod args;
mod create;

use args::CmdArgs;
use clap::Parser;

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

        args::Cmd::Build => {
            println!("we dont build things yet sorgy!");
        }

        args::Cmd::Init => {
            println!("hmmmmmmmmmmm, no!");
        }
    }
}
