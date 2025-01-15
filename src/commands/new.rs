use clap::Args;
use regex::Regex;

use crate::commands::checkout::CheckoutCommandArgs;
use crate::error::ErrorKind;
use super::{checkout::{checkout, open_repo}, result::CmdResult, stash::{pop_stash, stash}, update::{update, UpdateArgs}, Error};

#[derive(Args, Debug)]
pub struct NewCommandArgs {
    pub message: String,
}

pub fn format_branch_name(message: &str) -> String {
    let invalid_chars_pattern = Regex::new(r"[ ~^:?*\[\]\\`{}<>/\.\.@\|]+").unwrap();
    let alphanumeric_message = invalid_chars_pattern.replace_all(message, "_").to_string();
    let date = chrono::Local::now().format("%Y%m%d").to_string();
    let branch_name = format!("{}-{}", date, alphanumeric_message);
    branch_name
}

pub fn new(args: &NewCommandArgs) -> CmdResult<()> {
    let mut repo = open_repo()?;
    let branch_name = format_branch_name(&args.message);

    // Check if there are any changes to stash
    if repo.statuses(None).unwrap().len() > 0 {
        return Err(Error::new(
            ErrorKind::GitError,
            "There are changes in the working directory. Please commit or stash them before creating a new branch.".to_string(),
        ));
    }
    // Stash changes
    let should_stash = repo.statuses(None).unwrap().len() > 0;
    let stash_oid = match should_stash {
        false => None,
        _ => Some(stash(&mut repo, args.message.as_str())?),
    };

    // Create a branch
    repo.branch(
        &branch_name,
        &repo.head().unwrap().peel_to_commit().unwrap(),
        false,
    ).map_err(|e| {
        Error::new(
            ErrorKind::GitError,
            format!("Failed to create branch: {}", e),
        )
    })?;

    // Checkout the branch
    let checkout_args = CheckoutCommandArgs { branch: branch_name.clone() };
    checkout(&checkout_args)?;

    // Stash pop
    if let Some(oid) = stash_oid {
        pop_stash(&mut repo, oid)?;
    }

    // Update branch
    let update_args = UpdateArgs { message: Some(args.message.clone()), commit: true };
    update(&update_args)?;

    Ok(())
}