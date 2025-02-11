use super::result::CmdResult;
use clap::Args;
use lib::{branch::checkout_branch, repo::open_repo};

#[derive(Args, Debug)]
pub struct CheckoutCommandArgs {
    pub branch: String,
}

pub fn checkout(args: &CheckoutCommandArgs) -> CmdResult<()> {
    let repo = open_repo()?;
    checkout_branch(repo, &args.branch)?;
    Ok(())
}
