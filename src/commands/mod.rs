mod checkout;
mod delete;
mod down;
mod error;
mod new;
mod push;
mod raise_pr;
mod result;
mod stash;
mod sync;
mod trunk;
mod up;
mod update;
pub use error::{CommandError, CommandErrorKind};
pub use result::CmdResult;
use checkout::{checkout, CheckoutCommandArgs};
use delete::{delete, DeleteCommandArgs};
use down::{down, DownCommandArgs};
use new::{new, NewCommandArgs};
use push::{push, PushCommandArgs};
use raise_pr::{raise_pr, RaisePrCommandArgs};
use sync::{sync, SyncCommandArgs};
use trunk::{trunk, TrunkCommandArgs};
use up::{up, UpCommandArgs};
use update::{update, UpdateArgs};

use clap::{Parser, Subcommand};

pub trait ExecutableCommand {
    fn execute(&self) -> CmdResult<()>;
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
checkout - Checkout a branch
update - Reset HEAD to parent branch, stage changes, & make a commit with changes
delete - Deletes current branch,
up - Go to child
down - Go to parent
push - Push to remote (overwrite!)
raise-pr - Raise PR (requires plugin for remote)
 */

#[derive(Subcommand)]
pub enum Commands {
    Checkout(CheckoutCommandArgs),
    Trunk(TrunkCommandArgs),
    New(NewCommandArgs),
    Update(UpdateArgs),
    Delete(DeleteCommandArgs),
    Up(UpCommandArgs),
    Down(DownCommandArgs),
    Push(PushCommandArgs),
    RaisePr(RaisePrCommandArgs),
    Sync(SyncCommandArgs),
}

impl Commands {
    pub fn new() -> Self {
        let args = Cli::parse();
        args.cmd
    }
}

impl ExecutableCommand for Commands {
    fn execute(&self) -> CmdResult<()> {
        match self {
            Commands::Checkout(args) => checkout(args),
            Commands::New(args) => new(args),
            Commands::Trunk(args) => trunk(args),
            Commands::Update(args) => update(args),
            Commands::Delete(args) => delete(args),
            Commands::Up(args) => up(args),
            Commands::Down(args) => down(args),
            Commands::Push(args) => push(args),
            Commands::RaisePr(args) => raise_pr(args),
            Commands::Sync(args) => sync(args),
        }
    }
}
