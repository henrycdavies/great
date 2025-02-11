use super::result::CmdResult;

use clap::Args;

#[derive(Args, Debug)]
pub struct PushCommandArgs {}

pub fn push(_args: &PushCommandArgs) -> CmdResult<()> {
    unimplemented!("new command not implemented")
}
