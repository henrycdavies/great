use crate::commands::{Commands, CommandExecutionError, ExecutableCommand};

pub struct Handler {}

impl Handler {
    pub fn handle_input(&self) -> Result<i32, CommandExecutionError> {
        let command = Commands::new();
        if let Ok(()) = command.execute() {
            return Ok(0);
        }
        return Err(CommandExecutionError {})
    }
}