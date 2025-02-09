pub enum StashErrorKind {
    GitError,
    SaveError,
    PopError,
    RetrievalError,
    Unknown,
}

pub struct StashError {
    kind: StashErrorKind,
    message: String,
}

impl StashError {
    pub fn new(kind: StashErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &StashErrorKind {
        &self.kind
    }
}

impl From<git2::Error> for StashError {
    fn from(err: git2::Error) -> Self {
        Self::new(
            StashErrorKind::GitError,
            format!("Git error: {}", err.message()),
        )
    }
}

pub type StashResult<T> = Result<T, StashError>;
