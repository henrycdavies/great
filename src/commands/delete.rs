use super::result::Result;

use clap::Args;

#[derive(Args, Debug)]
pub struct DeleteCommandArgs {}

pub fn delete(_args: &DeleteCommandArgs) -> Result<()> {
    unimplemented!("new command not implemented")
}