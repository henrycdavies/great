pub enum BranchErrorKind {
    NotFound,
    ConflictsDetected,
    Unknown,
}

pub struct BranchError {
    kind: BranchErrorKind,
    message: String,
}

impl BranchError {
    pub fn new(kind: BranchErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &BranchErrorKind {
        &self.kind
    }
}

pub type BranchResult<T> = Result<T, BranchError>;
