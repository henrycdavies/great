use super::result::Result;

use clap::Args;

#[derive(Args, Debug)]
pub struct UpdateArgs {}

pub fn update(_args: &UpdateArgs) -> Result<()> {
    unimplemented!("new command not implemented")
}