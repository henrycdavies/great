use super::result::CmdResult;

use clap::Args;

#[derive(Args, Debug)]
pub struct DownCommandArgs {}

pub fn down(_args: &DownCommandArgs) -> CmdResult<()> {
    unimplemented!("new command not implemented")
}
