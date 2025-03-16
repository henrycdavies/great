use super::result::CmdResult;

use super::checkout::{self, CheckoutCommandArgs};
use clap::Args;
use lib::branch::find_trunk_branch;
use lib::repo::open_repo;

#[derive(Args, Debug)]
pub struct TrunkCommandArgs {}

pub fn trunk(_args: &TrunkCommandArgs) -> CmdResult<()> {
    let repo = open_repo()?;
    let trunk_branch_name = find_trunk_branch(&repo)?;
    checkout::checkout(&CheckoutCommandArgs {
        branch: trunk_branch_name,
    })
}
