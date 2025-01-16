#[allow(unused_imports)]
use clap::{Command, Parser};
use handler::Handler;
mod commands;
pub mod error;
mod handler;
mod utils;

fn main() {
    let handler = Handler {};
    let res = handler.handle_input();
    match res {
        Ok(code) => std::process::exit(code),
        Err(_) => {
            std::process::exit(1);
        }
    }
}
