use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    GitError,
    IoError,
    InvalidInput,
}

// Define the custom error type.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

// Implement `Display` for `CommandExecutionError`.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command execution failed")
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::new(
            ErrorKind::GitError,
            format!("Git error: {}", err.message()),
        )
    }
}