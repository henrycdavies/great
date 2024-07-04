use std::io::Result;

use clap::Args;

#[derive(Args, Debug)]
pub struct EditArgs {}

pub fn edit(_args: &EditArgs) -> Result<()> {
    unimplemented!("new command not implemented")
}