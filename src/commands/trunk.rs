use std::{fs, io::{Error, Result}};

use clap::Args;
use git2::Repository;

#[derive(Args, Debug)]
pub struct TrunkCommandArgs {}

pub fn trunk(_args: &TrunkCommandArgs) -> Result<()> {
    let head = fs::read_to_string(".git/HEAD")?;
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("failed to open repository: {}", e);
            std::process::exit(1);
        },
    };
    let trunk_branch_name = head.trim_start_matches("ref: refs/heads/");
    match repo.checkout_head(None) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("failed to checkout branch '{}': {}", trunk_branch_name, e);
            Err(Error::new(std::io::ErrorKind::InvalidInput, "failed to checkout branch"))
        },
    }
}