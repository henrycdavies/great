#[allow(unused_imports)]
extern crate lib;
use handler::Handler;
mod commands;
mod handler;

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
