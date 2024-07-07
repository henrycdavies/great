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
    let (object, reference) = repo.revparse_ext(refname).map_err(|_| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("The branch '{}' was not found in the repository.", refname),
        )
    })?;
    
    if let Err(e) = repo.checkout_tree(&object, None) {
        match e.code() {
            git2::ErrorCode::Conflict {} => {
                return Err(Error::new(
                    ErrorKind::GitError,
                    format!("Conflicts detected in the {} branch. Please stash your changes, or reset/update your branch and retry.", refname),
                ));
            },
            _ => {
                return Err(Error::new(
                    ErrorKind::GitError,
                    format!("Unknown error when searching for : {}", e.message()),
                ));
            },
        }
    }
    
    if let Some(gref) = reference {
        repo.set_head(gref.name().unwrap()).map_err(|_| {
            Error::new(
                ErrorKind::GitError,
                format!("Failed to set HEAD to '{}'.", refname),
            )
        })?;
    }
    
    Err(Error::new(
        ErrorKind::GitError,
        format!("Failed to set HEAD to '{}'.", refname),
    ))
}