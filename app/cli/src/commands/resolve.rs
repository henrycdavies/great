use crate::lib::merge::handler::MergeHandler;

use super::CmdResult;
use clap::Args;
use lib::repo::{add_all, open_repo};

#[derive(Args, Debug)]
pub struct ResolveArgs {}

pub fn resolve(_args: &ResolveArgs) -> CmdResult<()> {
    /*
     * We should take stashed commit(s) and apply them onto new base.
     * */
    let repo = open_repo()?;
    add_all(&repo)?;
    let merge_handler = MergeHandler::new(&repo, None);
    merge_handler.continue_and_resolve_rebase()?;
    Ok(())
}
