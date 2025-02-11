mod result;

pub use result::{StackError, StackErrorKind, StackResult};

use git2::Repository;

pub struct Stack<'a> {
    repo: &'a Repository,
}

impl<'a> Stack<'a> {
    pub fn new(repo: &'a Repository) -> Self {
        return Stack { repo };
    }

    pub fn find_trunk(&self) -> StackResult<String> {
        for branch_name in &["main", "master"] {
            if self
                .repo
                .find_branch(branch_name, git2::BranchType::Local)
                .is_ok()
            {
                return Ok(branch_name.to_string());
            }
        }

        let head = self.repo.head().map_err(|e| {
            // Error::new(std::io::ErrorKind::InvalidInput, e)
            StackError::new(
                StackErrorKind::GitError,
                format!("Failed to get HEAD: {}", e),
            )
        })?;
        if let Some(name) = head.shorthand() {
            return Ok(name.to_string());
        }

        Err(StackError::new(
            StackErrorKind::InvalidInput,
            "Failed to determine trunk branch".to_string(),
        ))
    }
}
