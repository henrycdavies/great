use git2::Repository;

use super::result::{BranchError, BranchErrorKind, BranchResult};


pub fn find_trunk_branch(repo: &Repository) -> BranchResult<String> {
    for branch_name in &["main", "master"] {
        if repo
            .find_branch(branch_name, git2::BranchType::Local)
            .is_ok()
        {
            return Ok(branch_name.to_string());
        }
    }

    let head = repo.head().map_err(|e| {
        BranchError::new(
            BranchErrorKind::NotFound,
            format!("Failed to get HEAD: {}", e),
        )
    })?;
    if let Some(name) = head.shorthand() {
        return Ok(name.to_string());
    }

    Err(BranchError::new(
        BranchErrorKind::NotFound,
        "Failed to determine trunk branch".to_string(),
    ))
}
