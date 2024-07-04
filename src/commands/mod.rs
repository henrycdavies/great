mod error;
mod trunk;
mod new;
mod edit;
mod delete;
mod up;
mod down;
mod push;
mod raise_pr;
use delete::{delete, DeleteCommandArgs};
use down::{down, DownCommandArgs};
use edit::{edit, EditArgs};
pub use error::CommandExecutionError;
use new::{new, NewCommandArgs};
use push::{push, PushCommandArgs};
use raise_pr::{raise_pr, RaisePrCommandArgs};
use trunk::{trunk, TrunkCommandArgs};
use up::{up, UpCommandArgs};
use std::io::Result;

use clap::{Parser, Subcommand};

pub trait ExecutableCommand {
    fn execute(&self) -> Result<()>;
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}


/*
trunk - Go to trunk
new - Create a branch from current branch
edit - Reset HEAD to parent branch, stage changes, & make a commit with changes
delete - Deletes current branch, 
up - Go to child
down - Go to parent
push - Push to remote (overwrite!)
raise-pr - Raise PR (requires plugin for remote)
 */

#[derive(Subcommand)]
pub enum Commands {
    Trunk(TrunkCommandArgs),
    New(NewCommandArgs),
    Edit(EditArgs),
    Delete(DeleteCommandArgs),
    Up(UpCommandArgs),
    Down(DownCommandArgs),
    Push(PushCommandArgs),
    RaisePr(RaisePrCommandArgs),
}

impl Commands {
    pub fn new() -> Self {
        let args = Cli::parse();
        args.cmd
    }
}

impl ExecutableCommand for Commands {
    fn execute(&self) -> Result<()> {
        match self {
            Commands::New(args) => new(args),
            Commands::Trunk(args) => trunk(args),
            Commands::Edit(args) => edit(args),
            Commands::Delete(args) => delete(args),
            Commands::Up(args) => up(args),
            Commands::Down(args) => down(args),
            Commands::Push(args) => push(args),
            Commands::RaisePr(args) => raise_pr(args),
        }
    }
}