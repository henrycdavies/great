pub enum MergeErrorKind {
    ConflictHandleError,
}

pub struct MergeError {
    kind: MergeErrorKind,
    message: String,
}

impl MergeError {
    pub fn new(kind: MergeErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &MergeErrorKind {
        &self.kind
    }
}

impl From<git2::Error> for MergeError {
    fn from(err: git2::Error) -> Self {
        Self::new(
            MergeErrorKind::ConflictHandleError,
            format!("Git error: {}", err.message()),
        )
    }
}

pub type MergeResult<T> = Result<T, MergeError>;
