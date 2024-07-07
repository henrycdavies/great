use clap::Args;

use super::{checkout::open_repo, result::Result, Error};

#[derive(Args, Debug)]
pub struct NewCommandArgs {
    pub message: String,
}

pub fn format_branch_name(message: &str) -> String {
    let date = chrono::Local::now().format("%Y%m%d%");
    let alphanumeric_message = message
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    let branch_name = format!("{}-{}", date, alphanumeric_message);
    branch_name
}

pub fn new(args: &NewCommandArgs) -> Result<()> {
    let repo = open_repo()?;
    let branch_name = format_branch_name(&args.message);

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
    Ok(())
}