use std::io::stdout;

use clap::Args;
use git2::build::CheckoutBuilder;
use git2::{Config, Cred, MergeOptions, RemoteCallbacks, Repository};

use crate::utils::{merge::normal::three_way_merge, pull::pull_changes};
use crate::utils::merge::fast_forward::fast_forward_merge;

use super::{checkout::open_repo, result::CmdResult, trunk::find_trunk_branch};

#[derive(Args, Debug)]
pub struct SyncCommandArgs {}

pub fn sync(args: &SyncCommandArgs) -> CmdResult<()> {
    let repo = open_repo()?;

    // Configure auth & progress reporting
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|url, username_from_url, allowed_types| {
        let config = Config::open_default()?;

        // Try git credential manager first
        if let Ok(cred) = Cred::credential_helper(&config, url, username_from_url) {
            return Ok(cred);
        }

        if allowed_types.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
            if let (Ok(username_cfg_entry), Ok(password_cfg_entry)) = (config.get_entry("username"), config.get_entry("password")) {
                if let (Some(username), Some(password)) = (username_cfg_entry.value(), password_cfg_entry.value()) {
                    let cred = Cred::userpass_plaintext(username, password)?;
                    return Ok(cred);
                }
            }
        }

        Err(git2::Error::from_str("No valid credentials found"))
    });
    callbacks.transfer_progress(|stats| {
        println!("Received {}/{} objects", stats.received_objects(), stats.total_objects());
        true
    });

    // Pull changes
    let fetch_head = pull_changes(&repo, "origin", callbacks)?;

    // Merge changes
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let (analysis, _) = repo.merge_analysis(&[&fetch_commit])?;

    let trunk_branch = find_trunk_branch(&repo)?;
    let trunk_branch_ref = format!("refs/heads/{}", trunk_branch);
    if analysis.is_fast_forward() {
        // Fast-forward merge
        fast_forward_merge(&repo, trunk_branch_ref.as_str(), &fetch_commit)?;
    } else if analysis.is_normal() {
        let commit = repo.find_commit(fetch_commit.id())?;
        three_way_merge(&repo, &commit)?;
    } else if analysis.is_up_to_date() {
        println!("Trunk is up-to-date.")
    } else {
        // Merge conflict
        // Log out 
        eprintln!("Merge conflict. Resolve them.");
        handle_conflict(&repo)?
    }
    Ok(())
}

fn handle_conflict(repo: &Repository) -> Result<(), git2::Error> {
    let mut opts = MergeOptions::new();
    let mut checkout_opts = CheckoutBuilder::new();
    checkout_opts.safe();
    repo.merge_commits(
        &repo.find_commit(repo.head()?.target().unwrap())?,
        &repo.find_commit(repo.find_reference("FETCH_HEAD")?.target().unwrap())?,
        Some(&mut opts)
    )?;
    repo.checkout_head(Some(&mut checkout_opts))?;
    Ok(())
}