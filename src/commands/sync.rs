use clap::Args;
use git2::{Config, Cred, FetchOptions, RemoteCallbacks};

use super::{checkout::open_repo, error::ErrorKind, result::Result, trunk::find_trunk_branch, Error};

#[derive(Args, Debug)]
pub struct SyncCommandArgs {}

pub fn sync(args: &SyncCommandArgs) -> Result<()> {
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
    
    // Configure remote & fetch config
    let mut remote = repo.find_remote("origin")?;
    let branch_refspecs = ["refs/heads/*:refs/remotes/origin/*"];
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    fetch_options.prune(git2::FetchPrune::On);

    // Fetch
    remote.fetch(&branch_refspecs, Some(&mut fetch_options), None)?;

    // Merge fetched changes
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let (analysis, _) = repo.merge_analysis(&[&fetch_commit])?;

    let trunk_branch = find_trunk_branch(&repo)?;
    let trunk_branch_refname = format!("refs/heads/{}", trunk_branch);
    if analysis.is_fast_forward() {
        // Fast forward merg
        let mut reference = repo.find_reference(trunk_branch_refname.as_str())?;
        reference.set_target(fetch_commit.id(), "Fast-forward")?;
        repo.set_head(trunk_branch_refname.as_str())?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
    } else if analysis.is_normal() {
        // Normal merge
        let commit = repo.find_commit(fetch_commit.id())?;
        let mut index = repo.merge_commits(&repo.head()?.peel_to_commit()?, &commit, None)?;
        if index.has_conflicts() {
            return Err(Error::new(
                ErrorKind::GitError,
                "Merge conflict".to_string(),
            ));
        }
        let result_tree = repo.find_tree(index.write_tree_to(&repo)?)?;
        let signature = repo.signature()?;
        let parent_commit = repo.head()?.peel_to_commit()?;
        repo.commit(Some("HEAD"), &signature, &signature, "Merge commit", &result_tree, &[&parent_commit, &fetch_commit])?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
    } else {
        // Merge conflict
    }
    return Ok(())
}