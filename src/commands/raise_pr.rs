use super::result::CmdResult;

use clap::Args;

#[derive(Args, Debug)]
pub struct RaisePrCommandArgs {}

pub fn raise_pr(_args: &RaisePrCommandArgs) -> CmdResult<()> {
    unimplemented!("new command not implemented")
}