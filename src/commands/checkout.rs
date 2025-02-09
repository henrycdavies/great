use super::{error::{CommandErrorKind, CommandError}, result::CmdResult};
use clap::Args;
use git2::Repository;

#[derive(Args, Debug)]
pub struct CheckoutCommandArgs {
    pub branch: String,
}

pub fn open_repo() -> CmdResult<Repository> {
    Repository::open(".").map_err(|e| {
        CommandError::new(
            CommandErrorKind::GitError,
            format!("Failed to open repository: {}", e),
        )
    })
}

pub fn checkout(args: &CheckoutCommandArgs) -> CmdResult<()> {
    let repo = open_repo()?;
    let refname = &args.branch.trim();
    let (object, reference) = repo.revparse_ext(refname).map_err(|_| {
        CommandError::new(
            CommandErrorKind::InvalidInput,
            format!("The branch '{}' was not found in the repository.", refname),
        )
    })?;

    if let Err(e) = repo.checkout_tree(&object, None) {
        match e.code() {
            git2::ErrorCode::Conflict {} => {
                return Err(CommandError::new(
                    CommandErrorKind::GitError,
                    format!("Conflicts detected in the {} branch. Please stash your changes, or reset/update your branch and retry.", refname),
                ));
            }
            _ => {
                return Err(CommandError::new(
                    CommandErrorKind::GitError,
                    format!("Unknown error when searching for : {}", e.message()),
                ));
            }
        }
    }

    if let Some(branch_ref) = reference {
        repo.set_head(branch_ref.name().unwrap()).map_err(|_| {
            CommandError::new(
                CommandErrorKind::GitError,
                format!("Failed to set HEAD to '{}'.", refname),
            )
        })?;
        return Ok(());
    }

    Err(CommandError::new(
        CommandErrorKind::GitError,
        format!("Failed to set HEAD to '{}'.", refname),
    ))
}
