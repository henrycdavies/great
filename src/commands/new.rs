use clap::Args;
use regex::Regex;

use crate::commands::checkout::{self, CheckoutCommandArgs};

use super::{checkout::{open_repo, checkout}, result::Result, Error};

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

pub fn new(args: &NewCommandArgs) -> Result<()> {
    let repo = open_repo()?;
    let branch_name = format_branch_name(&args.message);
    println!("Branch name: {}", branch_name);

    // Create a branch
    repo.branch(
        &branch_name,
        &repo.head().unwrap().peel_to_commit().unwrap(),
        false,
    ).map_err(|e| {
        Error::new(
            super::error::ErrorKind::GitError,
            format!("Failed to create branch: {}", e),
        )
    })?;

    // Checkout the branch
    let checkout_args = CheckoutCommandArgs { branch: branch_name.clone() };
    checkout(&checkout_args)?;
    Ok(())
}