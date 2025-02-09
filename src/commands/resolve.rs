use crate::utils::merge::handler::MergeHandler;

use super::{checkout::open_repo, update::add_all, CmdResult};
use clap::Args;

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
