use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CmdArgs {
    #[clap(subcommand)]
    pub command: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    Build { path: Option<PathBuf> },

    Init,

    New(NewCommands),
}

#[derive(Debug, Args)]
pub struct NewCommands {
    pub name: String,
}
