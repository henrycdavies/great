use lib::remote::PullError;

use super::{error::CommandError, CommandErrorKind};

pub type CmdResult<T> = std::result::Result<T, CommandError>;

impl From<PullError> for CommandError {
    fn from(err: PullError) -> Self {
        Self::new(CommandErrorKind::PullError, err.message().to_string())
    }
}
