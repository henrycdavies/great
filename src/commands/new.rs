use std::io::Result;

use clap::Args;

#[derive(Args, Debug)]
pub struct NewCommandArgs {}

pub fn new(_args: &NewCommandArgs) -> Result<()> {
    unimplemented!("new command not implemented")
}