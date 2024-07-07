use clap::Args;

use super::result::Result;

#[derive(Args, Debug)]
pub struct NewCommandArgs {}

pub fn new(_args: &NewCommandArgs) -> Result<()> {
    unimplemented!("new command not implemented")
}