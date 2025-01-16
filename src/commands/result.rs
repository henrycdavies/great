use super::Error;

pub type CmdResult<T> = std::result::Result<T, Error>;
