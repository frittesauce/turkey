mod args;

use args::CmdArgs;
use clap::Parser;

fn main() {
    let args = CmdArgs::parse();
    match args.command {
        args::Cmd::New(commands) => {
            println!("bro think he can make new project named {}", commands.name);
        }
        args::Cmd::Build => {
            println!("we dont build things yet sorgy!");
        }
        args::Cmd::Init => {
            println!("hmmmmmmmmmmm, no!");
        }
    }
}
