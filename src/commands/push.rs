use super::result::Result;

use clap::Args;

#[derive(Args, Debug)]
pub struct PushCommandArgs {}

pub fn push(_args: &PushCommandArgs) -> Result<()> {
    unimplemented!("new command not implemented")
}