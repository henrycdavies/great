use std::io::Result;

use clap::Args;

#[derive(Args, Debug)]
pub struct TrunkCommandArgs {}

pub fn trunk(_args: &TrunkCommandArgs) -> Result<()> {
    unimplemented!("new command not implemented")
}