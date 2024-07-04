use std::{fs, io::{Error, Result}};

use clap::Args;
use git2::Repository;

#[derive(Args, Debug)]
pub struct TrunkCommandArgs {}

pub fn trunk(_args: &TrunkCommandArgs) -> Result<()> {
    let head = fs::read_to_string(".git/HEAD")?;
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("failed to open repository: {}", e);
            std::process::exit(1);
        },
    };
    let trunk_branch_name = head.trim_start_matches("ref: refs/heads/");
    println!("Checking out trunk branch: {}", trunk_branch_name);
    let (object, reference) = repo.revparse_ext(trunk_branch_name).expect("Object not found");
    repo.checkout_tree(&object, None).expect("Failed to checkout");

    match reference {
        Some(refname) => {
            repo.set_head(refname.name().unwrap());
            return Ok(())
        },
        None => {
            repo.set_head_detached(object.id());
            Err(Error::new(std::io::ErrorKind::Other, "No reference found"))
        }
    }.expect("Failed to set HEAD")
}