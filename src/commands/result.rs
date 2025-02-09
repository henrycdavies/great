use super::error::CommandError;

pub type CmdResult<T> = std::result::Result<T, CommandError>;
