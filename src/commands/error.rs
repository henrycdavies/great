use std::fmt;

// Define the custom error type.
#[derive(Debug)]
pub struct CommandExecutionError;

// Implement `Display` for `CommandExecutionError`.
impl fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command execution failed")
    }
}
