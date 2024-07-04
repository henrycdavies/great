#[allow(unused_imports)]
use clap::{Parser,Command};

use handler::Handler;

mod commands;
mod handler;

fn main() {
    let handler = Handler{};
    let res = handler.handle_input();
    match res {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    }
}
