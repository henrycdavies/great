use git2::Repository;

use super::{RepoError, RepoResult};

pub fn open_repo() -> RepoResult<Repository> {
    Repository::open(".").map_err(|e| {
        RepoError::new(
            super::RepoErrorKind::GitError,
            format!("Failed to open repository: {}", e),
        )
    })
}
