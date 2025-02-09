use crate::utils::stack::StackError;

use super::conflict::ConflictHandleError;

pub enum MergeErrorKind {
    ConflictHandleError,
    StackError,
    Unknown,
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

impl From<ConflictHandleError> for MergeError {
    fn from(err: ConflictHandleError) -> Self {
        Self::new(
            MergeErrorKind::ConflictHandleError,
            format!("Conflict handle error: {}", err.message()),
        )
    }
}

impl From<StackError> for MergeError {
    fn from(err: StackError) -> Self {
        Self::new(
            MergeErrorKind::StackError,
            format!("Conflict handle error: {}", err.message()),
        )
    }
}

pub type MergeResult<T> = Result<T, MergeError>;
