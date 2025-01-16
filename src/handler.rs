use crate::commands::{Commands, Error as CommandError, ExecutableCommand};
use std::io::{Error, Result};

pub struct Handler {}

impl Handler {
    pub fn handle_input(&self) -> Result<i32> {
        let command = Commands::new();
        command.execute().map_err(|e: CommandError| {
            eprintln!("Error: {}", e.message());
            Error::from_raw_os_error(1)
        })?;
        Ok(0)
    }
}
