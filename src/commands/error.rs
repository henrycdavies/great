use crate::utils::merge::result::MergeError;

pub enum CommandErrorKind {
    GitError,
    MergeError,
    InvalidInput,
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

impl From<git2::Error> for CommandError {
    fn from(err: git2::Error) -> Self {
        CommandError::new(CommandErrorKind::GitError, err.message().to_string())
    }
}

impl From<MergeError> for CommandError {
    fn from(err: MergeError) -> Self {
        CommandError::new(CommandErrorKind::MergeError, err.message().to_string())
    }
}
