use super::result::CmdResult;

use clap::Args;

#[derive(Args, Debug)]
pub struct DeleteCommandArgs {}

pub fn delete(_args: &DeleteCommandArgs) -> CmdResult<()> {
    unimplemented!("new command not implemented")
}