use std::io::Result;

use clap::Args;
use git2::Repository;

#[derive(Args, Debug)]
pub struct CheckoutCommandArgs {
    pub branch: String,
}

pub fn checkout(args: &CheckoutCommandArgs) -> Result<()> {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("failed to open repository: {}", e);
            std::process::exit(1);
        },
    };
    let refname = &args.branch;
    let (object, reference) = repo.revparse_ext(refname).expect("Object not found");
    
    repo.checkout_tree(&object, None)
        .expect("Failed to checkout");

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .expect("Failed to set HEAD");

    Ok(())
}