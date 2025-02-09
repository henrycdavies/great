pub enum StackErrorKind {
    GitError,
    InvalidInput,
}

pub struct StackError {
    kind: StackErrorKind,
    message: String,
}

impl StackError {
    pub fn new(kind: StackErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &StackErrorKind {
        &self.kind
    }
}

impl From<git2::Error> for StackError {
    fn from(err: git2::Error) -> Self {
        Self::new(
            StackErrorKind::GitError,
            format!("Git error: {}", err.message()),
        )
    }
}

pub type StackResult<T> = Result<T, StackError>;
