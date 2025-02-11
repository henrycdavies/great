use super::result::CmdResult;
use clap::Args;
use lib::repo::{add_all, new_commit, open_repo};

#[derive(Args, Debug)]
pub struct UpdateArgs {
    #[arg(short, long)]
    pub commit: bool,
    pub message: Option<String>,
}

pub fn update_with_new_commit(args: &UpdateArgs) -> CmdResult<()> {
    let repo = open_repo()?;
    add_all(&repo)?;

    new_commit(repo, args.message.clone());

    Ok(())
}

pub fn update(args: &UpdateArgs) -> CmdResult<()> {
    update_with_new_commit(args)
}
