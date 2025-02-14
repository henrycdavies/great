use super::error::CommandErrorKind;
use super::CommandError;
use super::{checkout::open_repo, result::CmdResult};

use super::checkout::{self, CheckoutCommandArgs};
use clap::Args;
use git2::Repository;

#[derive(Args, Debug)]
pub struct TrunkCommandArgs {}

pub fn trunk(_args: &TrunkCommandArgs) -> CmdResult<()> {
    let repo = open_repo()?;
    let trunk_branch_name = find_trunk_branch(&repo)?;
    checkout::checkout(&CheckoutCommandArgs {
        branch: trunk_branch_name,
    })
}

pub fn find_trunk_branch(repo: &Repository) -> CmdResult<String> {
    for branch_name in &["main", "master"] {
        if repo
            .find_branch(branch_name, git2::BranchType::Local)
            .is_ok()
        {
            return Ok(branch_name.to_string());
        }
    }

    let head = repo.head().map_err(|e| {
        // Error::new(std::io::ErrorKind::InvalidInput, e)
        CommandError::new(
            CommandErrorKind::GitError,
            format!("Failed to get HEAD: {}", e),
        )
    })?;
    if let Some(name) = head.shorthand() {
        return Ok(name.to_string());
    }

    Err(CommandError::new(
        CommandErrorKind::InvalidInput,
        "Failed to determine trunk branch".to_string(),
    ))
}
