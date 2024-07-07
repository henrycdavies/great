use super::{checkout::open_repo, error::ErrorKind, result::Result, Error};

use clap::Args;
use git2::Repository;

use super::checkout::{self, CheckoutCommandArgs};

#[derive(Args, Debug)]
pub struct TrunkCommandArgs {}

pub fn trunk(_args: &TrunkCommandArgs) -> Result<()> {
    let repo = open_repo()?;
    let trunk_branch_name = find_trunk_branch(&repo)?;
    checkout::checkout(&CheckoutCommandArgs { branch: trunk_branch_name })
}

pub fn find_trunk_branch(repo: &Repository) -> Result<String> {
    for branch_name in &["main", "master"] {
        if repo.find_branch(branch_name, git2::BranchType::Local).is_ok() {
            return Ok(branch_name.to_string());
        }
    }

    let head = repo.head().map_err(|e| {
        // Error::new(std::io::ErrorKind::InvalidInput, e)
        Error::new(
            ErrorKind::GitError,
            format!("Failed to get HEAD: {}", e),
        )
    })?;
    if let Some(name) = head.shorthand() {
        return Ok(name.to_string());
    }

    Err(Error::new(
        ErrorKind::InvalidInput,
        "Failed to determine trunk branch".to_string(),
    ))
}