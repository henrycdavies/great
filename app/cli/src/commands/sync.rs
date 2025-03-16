use crate::lib::merge::handler::MergeHandler;
use clap::Args;
use lib::{
    remote::{configure_callbacks, pull_changes},
    repo::open_repo,
};

use super::result::CmdResult;

#[derive(Args, Debug)]
pub struct SyncCommandArgs {}

pub fn sync(_args: &SyncCommandArgs) -> CmdResult<()> {
    let repo = open_repo()?;

    // Configure auth & progress reporting
    let callbacks = configure_callbacks();

    // Pull changes
    let fetch_head = pull_changes(&repo, "origin", callbacks)?;

    // Merge changes
    // TODO: We need to fix this. Currently it doesn't stash the locally made commit(s), then apply them on master.
    // It instead tries the opposite way round (merge remote onto local).
    let merge_handler = MergeHandler::new(&repo, Some(&fetch_head));

    merge_handler.try_merge()?;
    println!("Sync successful.");
    Ok(())
}
