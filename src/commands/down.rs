use std::io::Result;

use clap::Args;

#[derive(Args, Debug)]
pub struct DownCommandArgs {}

pub fn down(_args: &DownCommandArgs) -> Result<()> {
    unimplemented!("new command not implemented")
}