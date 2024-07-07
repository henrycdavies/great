use std::io::Error;

use crate::commands::{Commands, ExecutableCommand};

pub struct Handler {}

impl Handler {
    pub fn handle_input(&self) -> Result<i32, Error> {
        let command = Commands::new();
        command.execute().map_err(|e| {
            println!("Error: {:?}", e);
            Error::from_raw_os_error(1)
        })?;
        Ok(0)
    }
}