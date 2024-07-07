use clap::Args;
use git2::Repository;

use super::{error::ErrorKind, result::Result, Error};

#[derive(Args, Debug)]
pub struct CheckoutCommandArgs {
    pub branch: String,
}

pub fn open_repo() -> Result<Repository> {
    Repository::open(".").map_err(|e| {
        Error::new(
            ErrorKind::GitError,
            format!("Failed to open repository: {}", e),
        )
    })
}

pub fn checkout(args: &CheckoutCommandArgs) -> Result<()> {
    let repo = open_repo()?;
    let refname = &args.branch;
    let (object, reference) = repo.revparse_ext(refname).map_err(|e| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("Branch ref '{}' not found: {}", refname, e),
        )
    })?;
    
    if let Err(e) = repo.checkout_tree(&object, None) {
        match e.code() {
            git2::ErrorCode::Conflict {} => {
                return Err(Error::new(
                    ErrorKind::GitError,
                    "Checkout failed: Conflicts detected.".to_string(),
                ));
            },
            _ => {
                return Err(Error::new(
                    ErrorKind::GitError,
                    format!("Checkout failed: {}", e),
                ));
            },
        }
    }

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .expect("Failed to set HEAD");

    Ok(())
}