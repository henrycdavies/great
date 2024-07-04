use std::io::Result;

use clap::Args;

#[derive(Args, Debug)]
pub struct UpCommandArgs {}

pub fn up(_args: &UpCommandArgs) -> Result<()> {
    unimplemented!("new command not implemented")
}