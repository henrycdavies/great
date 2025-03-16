use git2::Repository;

use super::{BranchError, BranchErrorKind, BranchResult};

pub fn checkout_branch(repo: Repository, branch: &str) -> BranchResult<()> {
    let refname = branch.trim();
    let (object, reference) = repo.revparse_ext(refname).map_err(|_| {
        BranchError::new(
            BranchErrorKind::NotFound,
            format!("The branch '{}' was not found in the repository.", refname),
        )
    })?;

    if let Err(e) = repo.checkout_tree(&object, None) {
        match e.code() {
            git2::ErrorCode::Conflict {} => {
                return Err(BranchError::new(
                    BranchErrorKind::ConflictsDetected,
                    format!("Conflicts detected in the {} branch. Please stash your changes, or reset/update your branch and retry.", refname),
                ));
            }
            _ => {
                return Err(BranchError::new(
                    BranchErrorKind::Unknown,
                    format!("Unknown error when searching for : {}", e.message()),
                ));
            }
        }
    }

    if let Some(branch_ref) = reference {
        repo.set_head(branch_ref.name().unwrap()).map_err(|_| {
            BranchError::new(
                BranchErrorKind::Unknown,
                format!("Failed to set HEAD to '{}'.", refname),
            )
        })?;
        return Ok(());
    }

    Err(BranchError::new(
        BranchErrorKind::Unknown,
        format!("Failed to set HEAD to '{}'.", refname),
    ))
}
