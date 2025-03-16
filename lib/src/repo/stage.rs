use super::{RepoError, RepoErrorKind, RepoResult};

pub fn add_all(repo: &git2::Repository) -> RepoResult<()> {
    let mut index = repo
        .index()
        .map_err(|_| RepoError::new(RepoErrorKind::GitError, "Failed to get index".to_string()))?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    match index.write() {
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err(RepoError::new(
                RepoErrorKind::GitError,
                "Failed to write index".to_string(),
            ))
        }
    }
}
