use std::io::Result;

use clap::Args;

#[derive(Args, Debug)]
pub struct RaisePrCommandArgs {}

pub fn raise_pr(_args: &RaisePrCommandArgs) -> Result<()> {
    unimplemented!("new command not implemented")
}