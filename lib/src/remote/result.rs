pub enum PullErrorKind {
    NotFound,
    ConflictsDetected,
    Unknown,
}

pub struct PullError {
    kind: PullErrorKind,
    message: String,
}

impl PullError {
    pub fn new(kind: PullErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &PullErrorKind {
        &self.kind
    }
}

pub type PullResult<T> = Result<T, PullError>;

impl From<git2::Error> for PullError {
    fn from(err: git2::Error) -> Self {
        Self::new(PullErrorKind::Unknown, err.message().to_string())
    }
}
