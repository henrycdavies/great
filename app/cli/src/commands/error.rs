use lib::{branch::BranchError, repo::RepoError};

use crate::lib::{
    merge::{conflict::ConflictHandleError, result::MergeError},
    stash::result::StashError,
};

pub enum CommandErrorKind {
    GitError,
    MergeError,
    RepoHandleError,
    PullError,
}

pub struct CommandError {
    kind: CommandErrorKind,
    message: String,
}

impl CommandError {
    pub fn new(kind: CommandErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &CommandErrorKind {
        &self.kind
    }
}

impl From<BranchError> for CommandError {
    fn from(value: BranchError) -> Self {
        CommandError::new(
            CommandErrorKind::RepoHandleError,
            value.message().to_string(),
        )
    }
}

impl From<ConflictHandleError> for CommandError {
    fn from(value: ConflictHandleError) -> Self {
        CommandError::new(CommandErrorKind::GitError, value.message().to_string())
    }
}

impl From<MergeError> for CommandError {
    fn from(value: MergeError) -> Self {
        CommandError::new(CommandErrorKind::MergeError, value.message().to_string())
    }
}

impl From<RepoError> for CommandError {
    fn from(value: RepoError) -> Self {
        CommandError::new(
            CommandErrorKind::RepoHandleError,
            value.message().to_string(),
        )
    }
}

impl From<StashError> for CommandError {
    fn from(value: StashError) -> Self {
        CommandError::new(CommandErrorKind::GitError, value.message().to_string())
    }
}
