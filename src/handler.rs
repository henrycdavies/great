use crate::commands::{CommandError, CommandErrorKind, Commands, ExecutableCommand};
use std::io::{Error, Result};

pub struct Handler {}

impl Handler {
    pub fn handle_input(&self) -> Result<i32> {
        let command = Commands::new();
        command.execute().map_err(|e: CommandError| {
            match e.kind() {
                CommandErrorKind::GitError => {
                    eprintln!("Error: {}", e.message());
                    Error::from_raw_os_error(1)
                }
                _ => {
                    eprintln!("Error: {}", e.message());
                    Error::from_raw_os_error(1)
                }
            }
        })?;
        Ok(0)
    }
}
