use clap::Args;
use lib::repo::open_repo;
use regex::Regex;

use super::{
    checkout::checkout,
    error::CommandErrorKind,
    result::CmdResult,
    update::{update, UpdateArgs},
    CommandError,
};
use crate::{
    commands::checkout::CheckoutCommandArgs,
    lib::stash::{pop_stash, stash},
};

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
        return Err(CommandError::new(
            CommandErrorKind::GitError,
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
    )
    .map_err(|e| {
        CommandError::new(
            CommandErrorKind::GitError,
            format!("Failed to create branch: {}", e),
        )
    })?;

    // Checkout the branch
    let checkout_args = CheckoutCommandArgs {
        branch: branch_name.clone(),
    };
    checkout(&checkout_args)?;

    // Stash pop
    if let Some(oid) = stash_oid {
        pop_stash(&mut repo, oid)?;
    }

    // Update branch
    let update_args = UpdateArgs {
        message: Some(args.message.clone()),
        commit: true,
    };
    update(&update_args)?;

    Ok(())
}
