use super::result::CmdResult;

use clap::Args;

#[derive(Args, Debug)]
pub struct UpCommandArgs {}

pub fn up(_args: &UpCommandArgs) -> CmdResult<()> {
    unimplemented!("new command not implemented")
}
