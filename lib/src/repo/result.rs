pub enum RepoErrorKind {
    GitError,
    Unknown,
}

pub struct RepoError {
    kind: RepoErrorKind,
    message: String,
}

impl RepoError {
    pub fn new(kind: RepoErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &RepoErrorKind {
        &self.kind
    }
}

impl From<git2::Error> for RepoError {
    fn from(value: git2::Error) -> Self {
        Self::new(
            RepoErrorKind::GitError,
            format!("Git error: {}", value.message()),
        )
    }
}

pub type RepoResult<T> = Result<T, RepoError>;
